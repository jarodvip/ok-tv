package com.fongmi.android.tv.server;

import android.util.Base64;
import android.util.Log;

import com.fongmi.android.tv.api.loader.BaseLoader;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;

public final class ProxyHandler {

    private static final String TAG = "ProxyHandler";

    private ProxyHandler() {
    }

    public static String handle(String query, byte[] body) {
        try {
            Map<String, String> params = new HashMap<>();
            if (query != null && !query.isEmpty()) {
                for (String pair : query.split("&")) {
                    if (!pair.contains("=")) continue;
                    int index = pair.indexOf('=');
                    params.put(pair.substring(0, index), pair.substring(index + 1));
                }
            }

            String headersJson = params.remove("_headers");
            if (headersJson != null && !headersJson.isEmpty()) {
                try {
                    JSONObject headersObj = new JSONObject(headersJson);
                    Iterator<String> keys = headersObj.keys();
                    while (keys.hasNext()) {
                        String key = keys.next();
                        params.put(key, headersObj.optString(key, ""));
                    }
                } catch (JSONException e) {
                    Log.w(TAG, "failed to parse headers json", e);
                }
            }

            if (body != null && body.length > 0) {
                params.put("_body", new String(body));
            }

            Object[] rs = BaseLoader.get().proxy(params);
            return buildResponse(rs);
        } catch (Throwable e) {
            Log.w(TAG, "proxy handler failed", e);
            return errorJson(500, e.getMessage());
        }
    }

    private static String buildResponse(Object[] rs) {
        if (rs == null || rs.length == 0) {
            return errorJson(500, "Invalid proxy response");
        }

        int status = 500;
        String mime = "text/plain";
        byte[] bodyBytes = new byte[0];
        Map<String, String> respHeaders = new HashMap<>();

        try {
            if (rs[0] instanceof fi.iki.elonen.NanoHTTPD.Response response) {
                status = response.getStatus().getRequestStatus();
                mime = response.getMimeType();
                bodyBytes = readStream(response.getData());
            } else if (rs.length >= 3) {
                status = (Integer) rs[0];
                mime = (String) rs[1];
                bodyBytes = readStream((InputStream) rs[2]);
                if (rs.length > 3 && rs[3] instanceof Map) {
                    @SuppressWarnings("unchecked")
                    Map<String, String> h = (Map<String, String>) rs[3];
                    respHeaders.putAll(h);
                }
            }
        } catch (Throwable e) {
            Log.w(TAG, "failed to build proxy response", e);
        }

        return toJson(status, mime, respHeaders, bodyBytes);
    }

    private static byte[] readStream(InputStream is) {
        if (is == null) return new byte[0];
        try (ByteArrayOutputStream baos = new ByteArrayOutputStream()) {
            byte[] buf = new byte[8192];
            int len;
            while ((len = is.read(buf)) > 0) baos.write(buf, 0, len);
            return baos.toByteArray();
        } catch (Exception e) {
            return new byte[0];
        }
    }

    private static String errorJson(int status, String message) {
        Map<String, String> headers = new HashMap<>();
        return toJson(status, "text/plain", headers, message.getBytes());
    }

    private static String toJson(int status, String mime, Map<String, String> headers, byte[] body) {
        JSONObject json = new JSONObject();
        try {
            json.put("status", status);
            json.put("mime", mime);
            JSONObject headersObj = new JSONObject();
            for (Map.Entry<String, String> entry : headers.entrySet()) {
                headersObj.put(entry.getKey(), entry.getValue());
            }
            json.put("headers", headersObj);
            json.put("body", Base64.encodeToString(body, Base64.NO_WRAP));
        } catch (JSONException e) {
            Log.w(TAG, "failed to build proxy json", e);
        }
        return json.toString();
    }
}
