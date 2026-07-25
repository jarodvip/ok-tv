package com.fongmi.android.tv.util;

import android.util.Log;

public final class RustUtil {

    private static final String TAG = "RustUtil";
    private static volatile boolean inited;

    private RustUtil() {
    }

    public static synchronized void init() {
        if (inited) {
            Log.w(TAG, "already inited");
            return;
        }
        try {
            nativeInit();
            inited = true;
            Log.i(TAG, "rust util inited");
        } catch (Throwable e) {
            Log.w(TAG, "rust util init failed", e);
        }
    }

    public static String s2t(String text) {
        if (text == null) return null;
        try {
            return nativeS2t(text);
        } catch (Throwable e) {
            return text;
        }
    }

    public static String t2s(String text) {
        if (text == null) return null;
        try {
            return nativeT2s(text);
        } catch (Throwable e) {
            return text;
        }
    }

    public static byte[] hex2byte(String hex) {
        if (hex == null) return null;
        try {
            return nativeHex2byte(hex);
        } catch (Throwable e) {
            return null;
        }
    }

    public static String md5(String text) {
        if (text == null) return "";
        try {
            return nativeMd5(text);
        } catch (Throwable e) {
            return "";
        }
    }

    public static String cbcDecrypt(String data) {
        if (data == null) return "";
        try {
            return nativeCbcDecrypt(data);
        } catch (Throwable e) {
            return "";
        }
    }

    public static String digest(String userInfo, String header, String method, String uri) {
        if (userInfo == null || header == null || method == null || uri == null) return "";
        try {
            return nativeDigest(userInfo, header, method, uri);
        } catch (Throwable e) {
            return "";
        }
    }

    public static String substring(String text) {
        if (text == null) return null;
        try {
            return nativeSubstringOne(text);
        } catch (Throwable e) {
            return text;
        }
    }

    public static String substring(String text, int num) {
        if (text == null) return null;
        try {
            return nativeSubstring(text, num);
        } catch (Throwable e) {
            return text;
        }
    }

    public static boolean containOrMatch(String text, String pattern) {
        if (text == null || pattern == null) return false;
        try {
            return nativeContainOrMatch(text, pattern);
        } catch (Throwable e) {
            return false;
        }
    }

    public static String queryGet(String query, String key) {
        if (query == null || key == null) return null;
        try {
            return nativeQueryGet(query, key);
        } catch (Throwable e) {
            return null;
        }
    }

    public static String resolveUri(String baseUri, String refUri) {
        if (baseUri == null || refUri == null) return "";
        try {
            return nativeResolveUri(baseUri, refUri);
        } catch (Throwable e) {
            return "";
        }
    }

    private static native void nativeInit();
    private static native String nativeS2t(String text);
    private static native String nativeT2s(String text);
    private static native byte[] nativeHex2byte(String hex);
    private static native String nativeMd5(String text);
    private static native String nativeCbcDecrypt(String data);
    private static native String nativeDigest(String userInfo, String header, String method, String uri);
    private static native String nativeSubstring(String text, int num);
    private static native String nativeSubstringOne(String text);
    private static native boolean nativeContainOrMatch(String text, String pattern);
    private static native String nativeQueryGet(String query, String key);
    private static native String nativeResolveUri(String baseUri, String refUri);

    static {
        try {
            System.loadLibrary("tv_util");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_util", t);
        }
    }
}
