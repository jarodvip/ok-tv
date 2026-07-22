package com.fongmi.android.tv.server;

import android.util.Log;

public final class RustServer {

    private static final String TAG = "RustServer";
    private static volatile boolean started;
    private static volatile int port;

    private RustServer() {
    }

    public static synchronized boolean isStarted() {
        return started;
    }

    public static synchronized int getPort() {
        return port;
    }

    public static synchronized boolean start(int portStart, int portEnd) {
        if (started) {
            Log.w(TAG, "already started on " + port);
            return true;
        }

        try {
            int result = nativeStart(portStart, portEnd);
            if (result > 0) {
                port = result;
                started = true;
                Log.i(TAG, "rust server started on " + port);
                return true;
            } else {
                Log.w(TAG, "rust native start returned invalid port=" + result);
                return false;
            }
        } catch (UnsatisfiedLinkError e) {
            Log.w(TAG, "rust server native lib missing", e);
            return false;
        } catch (Throwable e) {
            Log.w(TAG, "rust server start failed", e);
            return false;
        }
    }

    public static synchronized void stop() {
        if (!started) {
            return;
        }

        try {
            nativeStop();
        } catch (Throwable e) {
            Log.w(TAG, "rust server stop failed", e);
        } finally {
            started = false;
            port = 0;
        }
    }

    private static native int nativeStart(int portStart, int portEnd);
    private static native void nativeStop();

    static {
        try {
            System.loadLibrary("tv_server");
        } catch (Throwable t) {
            Log.w(TAG, "failed to load libtv_server", t);
        }
    }
}
