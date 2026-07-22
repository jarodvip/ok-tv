package com.fongmi.android.tv.server;

import android.util.Log;

import com.github.catvod.utils.Path;

public final class DelFileHandler {

    private static final String TAG = "DelFileHandler";

    private DelFileHandler() {
    }

    public static String handle(String path) {
        if (path == null) return "OK";
        try {
            Path.clear(Path.root(path));
        } catch (Throwable e) {
            Log.w(TAG, "del file failed", e);
        }
        return "OK";
    }
}
