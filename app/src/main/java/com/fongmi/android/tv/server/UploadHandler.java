package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.utils.FileUtil;
import com.github.catvod.utils.Path;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public final class UploadHandler {

    private static final String TAG = "UploadHandler";

    private UploadHandler() {
    }

    public static String handle(String query, byte[] body) {
        try {
            String path = QueryUtil.get(query, "path");
            if (path == null) path = "";

            List<UploadItem> items = parseItems(body);
            for (UploadItem item : items) {
                File target = Path.root(path, item.filename);
                if (item.isZip) {
                    FileUtil.zipDecompress(item.file, target.getParentFile());
                } else {
                    Path.copy(item.file, target);
                }
            }
            return "OK";
        } catch (Throwable e) {
            Log.w(TAG, "upload handler failed", e);
            return "OK";
        }
    }

    private static List<UploadItem> parseItems(byte[] body) {
        List<UploadItem> items = new ArrayList<>();
        if (body == null || body.length == 0) return items;
        try {
            JSONArray array = new JSONArray(new String(body));
            for (int i = 0; i < array.length(); i++) {
                JSONObject obj = array.getJSONObject(i);
                UploadItem item = new UploadItem();
                item.filename = obj.optString("filename", "");
                item.tempPath = obj.optString("tempPath", "");
                item.isZip = item.filename.toLowerCase().endsWith(".zip");
                item.file = new File(item.tempPath);
                items.add(item);
            }
        } catch (JSONException e) {
            Log.w(TAG, "failed to parse upload items", e);
        }
        return items;
    }

    private static class UploadItem {
        String filename;
        String tempPath;
        boolean isZip;
        File file;
    }
}
