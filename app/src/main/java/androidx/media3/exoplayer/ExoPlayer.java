package androidx.media3.exoplayer;

import androidx.media3.common.Player;

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
    void setCameraMotionListener(androidx.media3.exoplayer.spherical.CameraMotionListener listener);
    void setImageOutput(androidx.media3.exoplayer.image.ImageOutput imageOutput);
    void setPauseAtEndOfMediaItems(boolean pauseAtEndOfMediaItems);
    androidx.media3.exoplayer.ExoPlaybackException getPlayerError();
    androidx.media3.exoplayer.analytics.AnalyticsCollector getAnalyticsCollector();
    void addAnalyticsListener(androidx.media3.exoplayer.analytics.AnalyticsListener listener);
    void removeAnalyticsListener(androidx.media3.exoplayer.analytics.AnalyticsListener listener);
    androidx.media3.common.util.Clock getClock();
}
