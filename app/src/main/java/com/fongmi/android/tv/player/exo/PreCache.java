package com.fongmi.android.tv.player.exo;

import androidx.media3.common.MediaItem;

public class PreCache {

    private MediaItem mediaItem;

    public void start(com.fongmi.android.tv.exoplayer.ExoPlayer player, MediaItem mediaItem) {
        this.mediaItem = mediaItem;
    }

    public void stop() {
        mediaItem = null;
    }

    public void release() {
        stop();
    }
}
