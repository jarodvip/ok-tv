package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.App;
import com.fongmi.android.tv.bean.Device;

public final class DeviceHandler {

    private static final String TAG = "DeviceHandler";

    private DeviceHandler() {
    }

    public static String handle() {
        try {
            Device device = Device.get();
            Log.i(TAG, "device=" + device);
            return App.gson().toJson(device);
        } catch (Throwable e) {
            Log.w(TAG, "device handler failed", e);
            return "{}";
        }
    }
}
