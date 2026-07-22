package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.bean.Config;
import com.fongmi.android.tv.bean.Device;
import com.fongmi.android.tv.bean.History;
import com.fongmi.android.tv.event.CastEvent;

public final class CastHandler {

    private static final String TAG = "CastHandler";

    private CastHandler() {
    }

    public static String handle(String configJson, String deviceJson, String historyJson) {
        try {
            Config config = Config.objectFrom(configJson);
            Device device = Device.objectFrom(deviceJson);
            History history = History.objectFrom(historyJson);
            CastEvent.post(Config.find(config), device, history);
        } catch (Throwable e) {
            Log.w(TAG, "cast handler failed", e);
        }
        return "OK";
    }
}
