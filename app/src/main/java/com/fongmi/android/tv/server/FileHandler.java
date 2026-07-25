package com.fongmi.android.tv.server;

import android.util.Base64;
import android.util.Log;

import com.fongmi.android.tv.utils.FileUtil;
import com.fongmi.android.tv.utils.Formatters;
import com.github.catvod.utils.Path;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.time.Instant;
import java.time.ZoneId;
import java.util.HashMap;
import java.util.Map;
import java.util.zip.CRC32;

import fi.iki.elonen.NanoHTTPD;
import fi.iki.elonen.NanoHTTPD.Response;

public final class FileHandler {

    private static final String TAG = "FileHandler";

    private FileHandler() {
    }

    public static String handle(String path, String rangeHeader) {
        if (path == null) return "OK";
        try {
            File file = Path.local(path.substring(5));
            if (file.isDirectory()) return getFolder(file);
            if (file.isFile()) return getFile(file, rangeHeader);
            return toJson(NanoHTTPD.Response.Status.NOT_FOUND.getRequestStatus(), "text/plain", new HashMap<>(), "file not found".getBytes());
        } catch (Throwable e) {
            Log.w(TAG, "file handler failed", e);
            return toJson(NanoHTTPD.Response.Status.INTERNAL_ERROR.getRequestStatus(), "text/plain", new HashMap<>(), e.getMessage().getBytes());
        }
    }

    private static String getFolder(File dir) {
        try {
            File rootDir = Path.root();
            String rootPath = rootDir.getAbsolutePath();
            org.json.JSONArray files = new org.json.JSONArray();
            for (File file : Path.list(dir)) {
                org.json.JSONObject obj = new org.json.JSONObject();
                obj.put("name", file.getName());
                obj.put("path", relativeTo(file, rootPath));
                long time = file.lastModified();
                String timeStr = Formatters.LOCAL_DATETIME.format(Instant.ofEpochMilli(time).atZone(ZoneId.systemDefault()));
                obj.put("time", timeStr);
                obj.put("dir", file.isDirectory() ? 1 : 0);
                files.put(obj);
            }
            org.json.JSONObject info = new org.json.JSONObject();
            info.put("parent", parentOf(dir, rootDir, rootPath));
            info.put("files", files);
            return toJson(NanoHTTPD.Response.Status.OK.getRequestStatus(), "application/json", new HashMap<>(), info.toString().getBytes());
        } catch (Throwable e) {
            Log.w(TAG, "folder handler failed", e);
            return toJson(NanoHTTPD.Response.Status.INTERNAL_ERROR.getRequestStatus(), "text/plain", new HashMap<>(), e.getMessage().getBytes());
        }
    }

    private static String getFile(File file, String rangeHeader) throws IOException {
        long fileLen = file.length();
        String etag = etag(file, fileLen);
        if (matchesEtag(rangeHeader, etag)) {
            return toJson(NanoHTTPD.Response.Status.NOT_MODIFIED.getRequestStatus(), NanoHTTPD.getMimeTypeForFile(file.getName()), new HashMap<>(), new byte[0]);
        }

        long[] rangeInfo = parseRange(fileLen, rangeHeader, etag);
        long start = rangeInfo[0];
        long end = rangeInfo[1];
        long length = rangeInfo[2];
        boolean valid = rangeInfo[3] == 1;

        if (!valid) {
            return toJson(NanoHTTPD.Response.Status.RANGE_NOT_SATISFIABLE.getRequestStatus(), "text/plain", new HashMap<>(), new byte[0]);
        }

        try (FileInputStream fis = new FileInputStream(file)) {
            long skipped = fis.skip(start);
            if (skipped != start) throw new IOException("Failed to skip desired number of bytes");
            byte[] body = readStream(fis, length);
            Map<String, String> headers = new HashMap<>();
            headers.put("Content-Range", "bytes " + start + "-" + end + "/" + fileLen);
            headers.put("Content-Length", String.valueOf(length));
            headers.put("Accept-Ranges", "bytes");
            headers.put("ETag", etag);
            int status = length < fileLen ? NanoHTTPD.Response.Status.PARTIAL_CONTENT.getRequestStatus() : NanoHTTPD.Response.Status.OK.getRequestStatus();
            return toJson(status, NanoHTTPD.getMimeTypeForFile(file.getName()), headers, body);
        }
    }

    private static boolean matchesEtag(String rangeHeader, String etag) {
        if (rangeHeader == null || rangeHeader.isEmpty()) return false;
        String ifRange = extractHeaderValue(rangeHeader, "if-range");
        return ifRange != null && (ifRange.equals("*") || ifRange.equals(etag));
    }

    private static long[] parseRange(long fileLen, String rangeHeader, String etag) {
        long start = 0;
        long end = fileLen - 1;
        String header = rangeHeader;
        String ifRange = extractHeaderValue(rangeHeader, "if-range");
        if (ifRange != null && !ifRange.equals(etag)) header = null;
        if (header != null && header.startsWith("bytes=")) {
            try {
                String[] parts = header.substring(6).split("-", 2);
                if (!parts[0].isEmpty()) start = Long.parseLong(parts[0]);
                if (parts.length > 1 && !parts[1].isEmpty()) end = Long.parseLong(parts[1]);
                if (start >= fileLen || start > end) return new long[]{0, 0, 0, 0};
            } catch (NumberFormatException e) {
                return new long[]{0, 0, 0, 0};
            }
        }
        if (end >= fileLen) end = fileLen - 1;
        long length = end - start + 1;
        return new long[]{start, end, length, 1};
    }

    private static String extractHeaderValue(String header, String name) {
        if (header == null) return null;
        for (String part : header.split(";")) {
            part = part.trim();
            if (part.startsWith(name + "=")) {
                return part.substring(name.length() + 1);
            }
        }
        return null;
    }

    private static byte[] readStream(InputStream is, long length) throws IOException {
        byte[] buffer = new byte[8192];
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        long remaining = length;
        int read;
        while (remaining > 0 && (read = is.read(buffer, 0, (int) Math.min(buffer.length, remaining))) != -1) {
            baos.write(buffer, 0, read);
            remaining -= read;
        }
        return baos.toByteArray();
    }

    private static String relativeTo(File file, String rootPath) {
        String path = file.getAbsolutePath();
        return path.startsWith(rootPath) ? path.substring(rootPath.length()) : "";
    }

    private static String parentOf(File dir, File rootDir, String rootPath) {
        if (dir.equals(rootDir)) return ".";
        File parent = dir.getParentFile();
        if (parent == null || parent.equals(rootDir)) return "";
        return relativeTo(parent, rootPath);
    }

    private static String etag(File file, long fileLen) {
        try {
            CRC32 crc = new CRC32();
            crc.update((file.getAbsolutePath() + file.lastModified() + fileLen).getBytes());
            return Long.toHexString(crc.getValue());
        } catch (Exception e) {
            return "";
        }
    }

    private static String toJson(int status, String mime, Map<String, String> headers, byte[] body) {
        try {
            JSONObject json = new JSONObject();
            json.put("status", status);
            json.put("mime", mime);
            JSONObject headersObj = new JSONObject();
            for (Map.Entry<String, String> entry : headers.entrySet()) {
                headersObj.put(entry.getKey(), entry.getValue());
            }
            json.put("headers", headersObj);
            json.put("body", Base64.encodeToString(body, Base64.NO_WRAP));
            return json.toString();
        } catch (JSONException e) {
            Log.w(TAG, "failed to build file response json", e);
            return toJson(status, mime, headers, body);
        } catch (Throwable e) {
            Log.w(TAG, "failed to build file response json", e);
            return "";
        }
    }
}
