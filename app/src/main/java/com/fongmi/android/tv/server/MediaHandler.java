package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.App;
import com.fongmi.android.tv.player.PlayerManager;
import com.fongmi.android.tv.server.Server;
import com.fongmi.android.tv.service.PlaybackService;
import com.google.gson.JsonObject;

import java.util.concurrent.CompletableFuture;

public final class MediaHandler {

    private static final String TAG = "MediaHandler";

    private MediaHandler() {
    }

    public static String handle() {
        PlaybackService service = PlaybackService.isRunning() ? Server.get().getService() : null;
        if (service == null) return "{}";
        try {
            CompletableFuture<String> future = new CompletableFuture<>();
            App.post(() -> future.complete(build(service.player()).toString()));
            return future.get();
        } catch (Throwable e) {
            Log.w(TAG, "media handler failed", e);
            return "{}";
        }
    }

    private static JsonObject build(PlayerManager player) {
        if (player.isReleased()) return new JsonObject();
        androidx.media3.common.MediaItem item = player.getCurrentMediaItem();
        androidx.media3.common.MediaMetadata meta = item != null ? item.mediaMetadata : androidx.media3.common.MediaMetadata.EMPTY;
        JsonObject result = new JsonObject();
        result.addProperty("state", getState(player));
        result.addProperty("speed", player.getSpeed());
        result.addProperty("duration", player.getDuration());
        result.addProperty("position", player.getPosition());
        result.addProperty("url", getString(player.getUrl()));
        result.addProperty("title", getString(meta.title));
        result.addProperty("artist", getString(meta.artist));
        result.addProperty("artwork", getString(meta.artworkUri));
        return result;
    }

    private static int getState(PlayerManager player) {
        if (player.isPlaying()) return 3;
        int state = player.getPlaybackState();
        if (state == androidx.media3.common.Player.STATE_BUFFERING) return 6;
        if (state == androidx.media3.common.Player.STATE_READY) return 2;
        return 1;
    }

    private static String getString(CharSequence text) {
        return text != null ? text.toString() : "";
    }

    private static String getString(android.net.Uri uri) {
        return uri != null ? uri.toString() : "";
    }
}
