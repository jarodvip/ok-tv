package com.fongmi.android.tv.net;

import android.util.Log;

public final class RustNet {

    private static final String TAG = "RustNet";
    private static volatile boolean inited;
    private static volatile String proxyCache;
    private static volatile String cachedHost;

    private RustNet() {}

    public static synchronized void init(String rulesJson) {
        if (inited) {
            Log.w(TAG, "already inited");
            return;
        }
        try {
            nativeInit(rulesJson);
            inited = true;
            proxyCache = null;
            Log.i(TAG, "rust net inited");
        } catch (Throwable e) {
            Log.w(TAG, "rust net init failed", e);
        }
    }

    public static synchronized void reset() {
        inited = false;
        proxyCache = null;
    }

    public static String resolveProxy(String host) {
        if (host == null) return null;
        try {
            return nativeResolveProxy(host);
        } catch (Throwable e) {
            Log.w(TAG, "resolveProxy failed", e);
            return null;
        }
    }

    public static boolean shouldBlock(String url) {
        if (url == null) return false;
        try {
            return nativeShouldBlock(url);
        } catch (Throwable e) {
            Log.w(TAG, "shouldBlock failed", e);
            return false;
        }
    }

    public static String injectHeaders(String host, String headersJson) {
        if (host == null || headersJson == null) return headersJson;
        try {
            return nativeInjectHeaders(host, headersJson);
        } catch (Throwable e) {
            Log.w(TAG, "injectHeaders failed", e);
            return headersJson;
        }
    }

    public static String resolveProxyCached(String host) {
        if (host == null) return null;
        if (proxyCache != null && host.equals(cachedHost)) return proxyCache;
        String resolved = resolveProxy(host);
        if (resolved != null && !resolved.isEmpty()) {
            cachedHost = host;
            proxyCache = resolved;
        }
        return resolved;
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
