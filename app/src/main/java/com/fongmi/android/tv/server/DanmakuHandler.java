package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.service.PlaybackService;
import com.fongmi.android.tv.server.Server;

public final class DanmakuHandler {

    private static final String TAG = "DanmakuHandler";

    private DanmakuHandler() {
    }

    public static String handle(String text) {
        if (text == null) return "OK";
        PlaybackService service = PlaybackService.isRunning() ? Server.get().getService() : null;
        if (service == null) return "OK";
        try {
            service.player().sendDanmaku(text);
        } catch (Throwable e) {
            Log.w(TAG, "danmaku handler failed", e);
        }
        return "OK";
    }
}
