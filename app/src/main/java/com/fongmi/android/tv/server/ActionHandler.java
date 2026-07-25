package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.App;
import com.fongmi.android.tv.service.PlaybackService;

import java.util.Map;

public final class ActionHandler {

    private static final String TAG = "ActionHandler";

    private ActionHandler() {
    }

    public static String handle(String query) {
        try {
            String doValue = QueryUtil.get(query, "do");
            if ("control".equals(doValue)) return control(QueryUtil.get(query, "type"));
            if ("push".equals(doValue)) return push(query);
            if ("refresh".equals(doValue)) return refresh(query);
            if ("setting".equals(doValue)) return setting(query);
            if ("file".equals(doValue)) return file(QueryUtil.get(query, "path"));
            if ("search".equals(doValue)) return search(QueryUtil.get(query, "word"));
            if ("danmaku".equals(doValue)) return danmaku(QueryUtil.get(query, "text"));
            if ("cast".equals(doValue)) return cast(query);
            if ("sync".equals(doValue)) return sync(query);
            return "OK";
        } catch (Throwable e) {
            Log.w(TAG, "action handler failed", e);
            return "OK";
        }
    }

    private static String control(String type) {
        PlaybackService service = PlaybackService.isRunning() ? com.fongmi.android.tv.server.Server.get().getService() : null;
        if (service == null || type == null) return "OK";
        App.post(() -> {
            switch (type) {
                case "play" -> service.player().play();
                case "pause" -> service.player().pause();
                case "stop" -> service.dispatchStop();
                case "replay" -> service.dispatchReplay();
                case "prev" -> service.dispatchPrev();
                case "next" -> service.dispatchNext();
                case "repeat" -> service.dispatchRepeat();
            }
        });
        return "OK";
    }

    private static String push(String query) {
        String url = QueryUtil.get(query, "url");
        if (url == null) return "OK";
        App.post(() -> com.fongmi.android.tv.event.ServerEvent.push(url));
        return "OK";
    }

    private static String refresh(String query) {
        String type = QueryUtil.get(query, "type");
        if (type == null) return "OK";
        switch (type) {
            case "live" -> com.fongmi.android.tv.event.RefreshEvent.live();
            case "detail" -> com.fongmi.android.tv.event.RefreshEvent.detail();
            case "player" -> com.fongmi.android.tv.event.RefreshEvent.player();
            case "category" -> com.fongmi.android.tv.event.RefreshEvent.category();
            case "subtitle" -> com.fongmi.android.tv.event.RefreshEvent.subtitle(QueryUtil.get(query, "path"));
            case "danmaku" -> com.fongmi.android.tv.event.RefreshEvent.danmaku(QueryUtil.get(query, "path"));
            case "vod" -> {
                String json = QueryUtil.get(query, "json");
                if (json != null) com.fongmi.android.tv.event.RefreshEvent.vod(com.fongmi.android.tv.bean.Vod.objectFrom(json));
            }
        }
        return "OK";
    }

    private static String setting(String query) {
        String text = QueryUtil.get(query, "text");
        String name = QueryUtil.get(query, "name");
        if (text != null) com.fongmi.android.tv.event.ServerEvent.setting(text, name);
        return "OK";
    }

    private static String file(String path) {
        return FileHandler.handle(path, "");
    }

    private static String search(String word) {
        return SearchHandler.handle(word);
    }

    private static String danmaku(String text) {
        return DanmakuHandler.handle(text);
    }

    private static String cast(String query) {
        return CastHandler.handle(QueryUtil.get(query, "config"), QueryUtil.get(query, "device"), QueryUtil.get(query, "history"));
    }

    private static String sync(String query) {
        return SyncHandler.handle(QueryUtil.toMap(query));
    }
}
