package com.fongmi.android.tv.api.parser;

import android.util.Log;

public final class RustParser {

    private static final String TAG = "RustParser";
    private static volatile boolean inited;

    private RustParser() {
    }

    public static synchronized boolean init() {
        if (inited) return true;
        try {
            nativeInit();
            inited = true;
            Log.i(TAG, "rust parser inited");
            return true;
        } catch (Throwable e) {
            Log.w(TAG, "rust parser init failed", e);
            return false;
        }
    }

    public static String parse(String text) {
        if (text == null) return "[]";
        try {
            return nativeParse(text);
        } catch (Throwable e) {
            return "[]";
        }
    }

    private static native void nativeInit();
    private static native String nativeParse(String text);

    static {
        try {
            System.loadLibrary("tv_parse");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_parse", t);
        }
    }
}
