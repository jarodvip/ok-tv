package com.fongmi.android.tv.net;

import android.util.Log;

import org.json.JSONException;
import org.json.JSONObject;

public final class RustDns {

    private static final String TAG = "RustDns";
    private static volatile boolean inited;

    private RustDns() {
    }

    public static synchronized void init(String configJson) {
        if (inited) {
            Log.w(TAG, "already inited");
            return;
        }

        try {
            nativeInit(configJson);
            inited = true;
            Log.i(TAG, "rust dns inited");
        } catch (Throwable e) {
            Log.w(TAG, "rust dns init failed", e);
        }
    }

    public static synchronized void reset() {
        inited = false;
    }

    public static String resolveHost(String host) {
        if (host == null) return null;
        try {
            return nativeResolveHost(host);
        } catch (Throwable e) {
            Log.w(TAG, "resolveHost failed", e);
            return null;
        }
    }

    private static native void nativeInit(String configJson);
    private static native String nativeResolveHost(String host);

    static {
        try {
            System.loadLibrary("tv_dns");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_dns", t);
        }
    }
}
