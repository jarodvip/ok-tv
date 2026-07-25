package com.github.catvod.net;

import android.util.Log;

public final class RustNet {

    private static final String TAG = "RustNet";
    private static volatile boolean inited;

    private RustNet() {}

    public static synchronized void init(String rulesJson) {
        if (inited) return;
        try {
            nativeInit(rulesJson);
            inited = true;
        } catch (Throwable e) {
            Log.w(TAG, "rust net init failed", e);
        }
    }

    public static String resolveProxy(String host) {
        if (host == null) return null;
        try {
            return nativeResolveProxy(host);
        } catch (Throwable e) {
            return null;
        }
    }

    public static boolean shouldBlock(String url) {
        if (url == null) return false;
        try {
            return nativeShouldBlock(url);
        } catch (Throwable e) {
            return false;
        }
    }

    public static String injectHeaders(String host, String headersJson) {
        if (host == null || headersJson == null) return headersJson;
        try {
            return nativeInjectHeaders(host, headersJson);
        } catch (Throwable e) {
            return headersJson;
        }
    }

    private static native void nativeInit(String rulesJson);
    private static native String nativeResolveProxy(String host);
    private static native boolean nativeShouldBlock(String url);
    private static native String nativeInjectHeaders(String host, String headersJson);

    static {
        try {
            System.loadLibrary("tv_net");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_net", t);
        }
    }
}
