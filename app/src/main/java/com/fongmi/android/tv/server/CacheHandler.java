package com.fongmi.android.tv.server;

import android.text.TextUtils;

import com.github.catvod.utils.Prefers;

public final class CacheHandler {

    private CacheHandler() {
    }

    public static String handle(String query) {
        String action = QueryUtil.get(query, "do");
        String rule = QueryUtil.get(query, "rule");
        String key = QueryUtil.get(query, "key");
        if (TextUtils.isEmpty(action) || TextUtils.isEmpty(key)) return "OK";
        String cacheKey = buildKey(rule, key);
        return switch (action) {
            case "get" -> get(cacheKey);
            case "set" -> set(cacheKey, QueryUtil.get(query, "value"));
            case "del" -> del(cacheKey);
            default -> "OK";
        };
    }

    private static String buildKey(String rule, String key) {
        return "cache_" + (TextUtils.isEmpty(rule) ? "" : rule + "_") + key;
    }

    private static String get(String key) {
        return Prefers.getString(key, "");
    }

    private static String set(String key, String value) {
        if (value != null) Prefers.put(key, value);
        return "OK";
    }

    private static String del(String key) {
        Prefers.remove(key);
        return "OK";
    }
}
