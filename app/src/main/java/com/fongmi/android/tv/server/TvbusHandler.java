package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.api.config.LiveConfig;

public final class TvbusHandler {

    private static final String TAG = "TvbusHandler";

    private TvbusHandler() {
    }

    public static String handle(String query) {
        try {
            String resp = LiveConfig.getResp();
            Log.i(TAG, "tvbus resp len=" + (resp == null ? -1 : resp.length()));
            return resp == null ? "" : resp;
        } catch (Throwable e) {
            Log.w(TAG, "tvbus handler failed", e);
            return "";
        }
    }
}
