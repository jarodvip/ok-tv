package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.event.ServerEvent;

public final class SearchHandler {

    private static final String TAG = "SearchHandler";

    private SearchHandler() {
    }

    public static String handle(String word) {
        if (word == null) return "OK";
        try {
            ServerEvent.search(word);
        } catch (Throwable e) {
            Log.w(TAG, "search handler failed", e);
        }
        return "OK";
    }
}
