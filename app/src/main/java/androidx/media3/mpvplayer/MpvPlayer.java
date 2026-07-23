package androidx.media3.mpvplayer;

import android.content.Context;

public final class MpvPlayer {

    public static boolean isAvailable() { return false; }

    public static final class Builder {
        private final Context context;
        private int decode;
        private MpvPlayerConfig config;

        public Builder(Context context) { this.context = context; }
        public Builder setDecode(int decode) { this.decode = decode; return this; }
        public Builder setConfig(MpvPlayerConfig config) { this.config = config; return this; }
        public MpvPlayer build() { return new MpvPlayer(); }
    }

    private MpvPlayer() {}

    public void addListener(Object listener) {}
    public void removeListener(Object listener) {}
    public void setMediaItem(androidx.media3.common.MediaItem mediaItem, long startPositionMs) {}
    public void prepare() {}
    public void play() {}
    public void release() {}
    public void addSubtitle(Object config) {}
    public void setDecode(int decode) {}
    public void setSubtitleOptions(MpvPlayerConfig config) {}
    public void setDanmakuConfig(Object config) {}
    public int getCurrentMediaItemIndex() { return 0; }
    public androidx.media3.common.MediaItem getCurrentMediaItem() { return null; }
    public long getDuration() { return 0; }
    public long getCurrentPosition() { return 0; }
    public boolean isPlaying() { return false; }
    public int getPlaybackState() { return 0; }
    public void setPlayWhenReady(boolean playWhenReady) {}
}
