package com.fongmi.android.tv.exoplayer.source.preload;

import androidx.media3.common.MediaItem;
import androidx.media3.common.PriorityTaskManager;
import androidx.media3.exoplayer.ExoPlayer;
import androidx.media3.exoplayer.source.MediaSource;

public final class DiskPreloadManager {

    public static final class Builder {
        private final Object cache;
        private final MediaSource.Factory upstreamFactory;
        private final Object renderersFactory;
        private PriorityTaskManager priorityTaskManager;

        public Builder(Object cache, MediaSource.Factory upstreamFactory, Object renderersFactory) {
            this.cache = cache;
            this.upstreamFactory = upstreamFactory;
            this.renderersFactory = renderersFactory;
        }

        public Builder setPriorityTaskManager(PriorityTaskManager priorityTaskManager) { this.priorityTaskManager = priorityTaskManager; return this; }
        public DiskPreloadManager build() { return new DiskPreloadManager(); }
    }

    public static final class Options {
        private long durationMs;
        private int maxThreads;

        private Options() {}

        public static Options builder() { return new Options(); }
        public Options setDurationMs(long durationMs) { this.durationMs = durationMs; return this; }
        public Options setMaxThreads(int maxThreads) { this.maxThreads = maxThreads; return this; }
        public Options build() { return this; }
    }

    private DiskPreloadManager() {}

    public void start(com.fongmi.android.tv.exoplayer.ExoPlayer player, MediaItem mediaItem, Options options) {}
    public void release() {}
}
