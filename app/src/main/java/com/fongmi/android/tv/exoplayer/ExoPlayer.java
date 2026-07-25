package com.fongmi.android.tv.exoplayer;

import androidx.media3.common.Effect;
import androidx.media3.common.MediaItem;
import androidx.media3.common.Player;
import androidx.media3.exoplayer.image.ImageOutput;
import androidx.media3.exoplayer.source.MediaSource;
import androidx.media3.exoplayer.source.ShuffleOrder;

import java.util.List;

public interface ExoPlayer extends Player {

    void prepare();
    void prepare(boolean resetPosition, boolean resetState);
    void setMediaSource(androidx.media3.exoplayer.source.MediaSource mediaSource);
    void setMediaSource(androidx.media3.exoplayer.source.MediaSource mediaSource, boolean resetPosition);
    void setMediaSource(androidx.media3.exoplayer.source.MediaSource mediaSource, long startPositionMs);
    void addMediaSource(androidx.media3.exoplayer.source.MediaSource mediaSource);
    void addMediaSources(java.util.List<? extends androidx.media3.exoplayer.source.MediaSource> mediaSources);
    void setShuffleOrder(androidx.media3.exoplayer.source.ShuffleOrder shuffleOrder);
    void setAudioSessionId(int audioSessionId);
    void setSkipSilenceEnabled(boolean skipSilenceEnabled);
    void setVideoEffects(java.util.List<? extends androidx.media3.common.Effect> videoEffects);
    void setVideoScalingMode(int videoScalingMode);
    void setVideoChangeFrameRateStrategy(int videoChangeFrameRateStrategy);
    void setCameraMotionListener(Object listener);
    void setImageOutput(androidx.media3.exoplayer.image.ImageOutput imageOutput);
    void setPauseAtEndOfMediaItems(boolean pauseAtEndOfMediaItems);
    androidx.media3.exoplayer.ExoPlaybackException getPlayerError();
    androidx.media3.exoplayer.analytics.AnalyticsCollector getAnalyticsCollector();
    void addAnalyticsListener(androidx.media3.exoplayer.analytics.AnalyticsListener listener);
    void removeAnalyticsListener(androidx.media3.exoplayer.analytics.AnalyticsListener listener);
    androidx.media3.common.util.Clock getClock();

    void setMediaItem(androidx.media3.common.MediaItem mediaItem, long startPositionMs);
    void addSubtitle(Object config);
    void setDecode(int decode);
    void mute();
    void setHandleAudioBecomingNoisy(boolean handleAudioBecomingNoisy);
    void setSubtitleOptions(Object config);
    void setDanmakuConfig(Object config);
    float getSpeed();
    String getUrl();
    long getPosition();
    android.net.Uri getSelectedDanmakuUri();
    boolean isReleased();
    int getCurrentMediaItemIndex();
    long getDuration();
    long getCurrentPosition();
    boolean isPlaying();
    int getPlaybackState();

    class Builder {
        public Builder(android.content.Context context) {}
        public Builder setTrackSelector(androidx.media3.exoplayer.trackselection.TrackSelector trackSelector) { return this; }
        public Builder setRenderersFactory(androidx.media3.exoplayer.RenderersFactory renderersFactory) { return this; }
        public Builder setMediaSourceFactory(androidx.media3.exoplayer.source.MediaSource.Factory mediaSourceFactory) { return this; }
        public ExoPlayer build() { return null; }
    }
}
