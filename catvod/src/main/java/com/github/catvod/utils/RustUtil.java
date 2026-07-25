package com.github.catvod.utils;

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

    public static String md5(String text) {
        if (text == null) return "";
        try {
            return nativeMd5(text);
        } catch (Throwable e) {
            return "";
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

    private static native void nativeInit();
    private static native String nativeMd5(String text);
    private static native byte[] nativeHex2byte(String hex);
    private static native String nativeSubstring(String text, int num);
    private static native String nativeSubstringOne(String text);
    private static native boolean nativeContainOrMatch(String text, String pattern);

    static {
        try {
            System.loadLibrary("tv_util");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_util", t);
        }
    }
}
