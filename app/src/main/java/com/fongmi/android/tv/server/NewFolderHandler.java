package com.fongmi.android.tv.server;

import android.util.Log;

import com.github.catvod.utils.Path;

public final class NewFolderHandler {

    private static final String TAG = "NewFolderHandler";

    private NewFolderHandler() {
    }

    public static String handle(String query) {
        String path = QueryUtil.get(query, "path");
        String name = QueryUtil.get(query, "name");
        if (path == null && name == null) return "OK";
        try {
            Path.root(path, name).mkdirs();
        } catch (Throwable e) {
            Log.w(TAG, "new folder failed", e);
        }
        return "OK";
    }
}
