package com.fongmi.android.tv.server;

import android.text.TextUtils;

import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;

public final class QueryUtil {

    private QueryUtil() {
    }

    public static String get(String query, String key) {
        if (TextUtils.isEmpty(query) || key == null) return null;
        for (String pair : query.split("&")) {
            if (pair.startsWith(key + "=")) {
                String value = pair.substring(key.length() + 1);
                return decode(value);
            }
        }
        return null;
    }

    public static Map<String, String> toMap(String query) {
        if (TextUtils.isEmpty(query)) return Collections.emptyMap();
        Map<String, String> map = new LinkedHashMap<>();
        for (String pair : query.split("&")) {
            if (!pair.contains("=")) continue;
            int index = pair.indexOf('=');
            map.put(pair.substring(0, index), decode(pair.substring(index + 1)));
        }
        return map;
    }

    private static String decode(String value) {
        if (value == null) return null;
        try {
            return URLDecoder.decode(value, "UTF-8");
        } catch (UnsupportedEncodingException e) {
            return value;
        }
    }
}
