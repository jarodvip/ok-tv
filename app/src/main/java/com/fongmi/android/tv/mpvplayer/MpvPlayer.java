package com.fongmi.android.tv.mpvplayer;

import android.content.Context;

import androidx.media3.common.AudioAttributes;
import androidx.media3.common.text.CueGroup;
import androidx.media3.common.DeviceInfo;
import androidx.media3.common.MediaItem;
import androidx.media3.common.MediaMetadata;
import androidx.media3.common.PlaybackException;
import androidx.media3.common.PlaybackParameters;
import androidx.media3.common.Player;
import androidx.media3.common.Timeline;
import androidx.media3.common.TrackSelectionParameters;
import androidx.media3.common.Tracks;
import androidx.media3.common.VideoSize;
import androidx.media3.common.util.Size;

import java.util.List;

public final class MpvPlayer implements Player {

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

    public void addListener(Player.Listener listener) {}
    public void removeListener(Player.Listener listener) {}
    public void setMediaItem(MediaItem mediaItem, long startPositionMs) {}
    public void prepare() {}
    public void prepare(boolean resetPosition, boolean resetState) {}
    public void play() {}
    public void release() {}
    public void addSubtitle(Object config) {}
    public void setDecode(int decode) {}
    public void setSubtitleOptions(MpvPlayerConfig config) {}
    public void setDanmakuConfig(Object config) {}
    public void setPlayWhenReady(boolean playWhenReady) {}
    public int getCurrentMediaItemIndex() { return 0; }
    public MediaItem getCurrentMediaItem() { return null; }
    public long getDuration() { return 0; }
    public long getCurrentPosition() { return 0; }
    public boolean isPlaying() { return false; }
    public int getPlaybackState() { return 0; }
    public void setMediaItems(java.util.List<MediaItem> mediaItems) {}
    public void setMediaItems(java.util.List<MediaItem> mediaItems, boolean resetPosition) {}
    public void setMediaItems(java.util.List<MediaItem> mediaItems, int startIndex, long startPositionMs) {}
    public void setMediaItem(MediaItem mediaItem) {}
    public void setMediaItem(MediaItem mediaItem, boolean resetPosition) {}
    public void addMediaItem(MediaItem mediaItem) {}
    public void addMediaItem(int index, MediaItem mediaItem) {}
    public void addMediaItems(java.util.List<MediaItem> mediaItems) {}
    public void addMediaItems(int index, java.util.List<MediaItem> mediaItems) {}
    public void moveMediaItem(int fromIndex, int toIndex) {}
    public void moveMediaItems(int fromIndex, int toIndex, int newIndex) {}
    public void removeMediaItems(java.util.List<MediaItem> mediaItems) {}
    public void removeMediaItem(int index) {}
    public void removeMediaItems(int fromIndex, int toIndex) {}
    public void replaceMediaItem(int index, MediaItem mediaItem) {}
    public void replaceMediaItems(int fromIndex, int toIndex, java.util.List<MediaItem> mediaItems) {}
    public void clearMediaItems() {}
    public boolean isCommandAvailable(int command) { return false; }
    public boolean canAdvertiseSession() { return false; }
    public Player.Commands getAvailableCommands() { return Player.Commands.EMPTY; }
    public boolean getPlayWhenReady() { return false; }
    public void setRepeatMode(int repeatMode) {}
    public int getRepeatMode() { return Player.REPEAT_MODE_OFF; }
    public void setShuffleModeEnabled(boolean shuffleModeEnabled) {}
    public boolean getShuffleModeEnabled() { return false; }
    public void seekToDefaultPosition() {}
    public void seekToDefaultPosition(int mediaItemIndex) {}
    public void seekTo(long positionMs) {}
    public void seekTo(int mediaItemIndex, long positionMs) {}
    public long getSeekBackIncrement() { return 0; }
    public void seekBack() {}
    public long getSeekForwardIncrement() { return 0; }
    public void seekForward() {}
    public boolean hasPrevious() { return false; }
    public boolean hasPreviousWindow() { return false; }
    public boolean hasPreviousMediaItem() { return false; }
    public void previous() {}
    public void seekToPreviousWindow() {}
    public void seekToPreviousMediaItem() {}
    public long getMaxSeekToPreviousPosition() { return 0; }
    public void seekToPrevious() {}
    public boolean hasNext() { return false; }
    public boolean hasNextWindow() { return false; }
    public boolean hasNextMediaItem() { return false; }
    public void next() {}
    public void seekToNextWindow() {}
    public void seekToNextMediaItem() {}
    public void seekToNext() {}
    public void setPlaybackSpeed(float speed) {}
    public MediaMetadata getMediaMetadata() { return null; }
    public MediaMetadata getPlaylistMetadata() { return null; }
    public void setPlaylistMetadata(MediaMetadata mediaMetadata) {}
    public Object getCurrentManifest() { return null; }
    public int getCurrentPeriodIndex() { return 0; }
    public int getCurrentWindowIndex() { return 0; }
    public int getNextWindowIndex() { return 0; }
    public int getNextMediaItemIndex() { return 0; }
    public int getPreviousWindowIndex() { return 0; }
    public int getPreviousMediaItemIndex() { return 0; }
    public MediaItem getMediaItemAt(int index) { return null; }
    public int getMediaItemCount() { return 0; }
    public long getCurrentLiveOffset() { return 0; }
    public PlaybackException getPlayerError() { return null; }
    public android.os.Looper getApplicationLooper() { return null; }
    public AudioAttributes getAudioAttributes() { return null; }
    public VideoSize getVideoSize() { return null; }
    public CueGroup getCurrentCues() { return null; }
    public DeviceInfo getDeviceInfo() { return null; }
    public void setAudioAttributes(AudioAttributes attributes, boolean handleAudioFocus) {}
    public PlaybackParameters getPlaybackParameters() { return null; }
    public void setPlaybackParameters(PlaybackParameters parameters) {}
    public Tracks getCurrentTracks() { return null; }
    public TrackSelectionParameters getTrackSelectionParameters() { return null; }
    public void setTrackSelectionParameters(TrackSelectionParameters parameters) {}
    public Timeline getCurrentTimeline() { return null; }
    public int getPlaybackSuppressionReason() { return Player.PLAYBACK_SUPPRESSION_REASON_NONE; }
    public void pause() {}
    public boolean isLoading() { return false; }
    public void stop() {}
    public long getBufferedPosition() { return 0; }
    public int getBufferedPercentage() { return 0; }
    public long getTotalBufferedDuration() { return 0; }
    public boolean isCurrentWindowDynamic() { return false; }
    public boolean isCurrentMediaItemDynamic() { return false; }
    public boolean isCurrentWindowLive() { return false; }
    public boolean isCurrentMediaItemLive() { return false; }
    public boolean isPlayingAd() { return false; }
    public int getCurrentAdGroupIndex() { return 0; }
    public int getCurrentAdIndexInAdGroup() { return 0; }
    public long getContentDuration() { return 0; }
    public long getContentPosition() { return 0; }
    public long getContentBufferedPosition() { return 0; }
    public void setVolume(float volume) {}
    public float getVolume() { return 1.0f; }
    public void clearVideoSurface() {}
    public void clearVideoSurface(android.view.Surface surface) {}
    public void setVideoSurface(android.view.Surface surface) {}
    public void setVideoSurfaceHolder(android.view.SurfaceHolder holder) {}
    public void clearVideoSurfaceHolder(android.view.SurfaceHolder holder) {}
    public void setVideoSurfaceView(android.view.SurfaceView view) {}
    public void clearVideoSurfaceView(android.view.SurfaceView view) {}
    public void setVideoTextureView(android.view.TextureView textureView) {}
    public void clearVideoTextureView(android.view.TextureView textureView) {}
    public Size getSurfaceSize() { return null; }
    public int getDeviceVolume() { return 0; }
    public boolean isDeviceMuted() { return false; }
    public void setDeviceVolume(int volume) {}
    public void setDeviceVolume(int volume, int flags) {}
    public void increaseDeviceVolume() {}
    public void increaseDeviceVolume(int flags) {}
    public void decreaseDeviceVolume() {}
    public void decreaseDeviceVolume(int flags) {}
    public void setDeviceMuted(boolean muted) {}
    public void setDeviceMuted(boolean muted, int flags) {}
    public void setPriorityTaskManager(androidx.media3.common.PriorityTaskManager priorityTaskManager) {}
    public long getTextOffsetMs() { return 0; }
    public void setTextOffsetMs(long textOffsetMs) {}
    public long getAudioOffsetMs() { return 0; }
    public void setAudioOffsetMs(long audioOffsetMs) {}
    public void setHandleAudioBecomingNoisy(boolean handleAudioBecomingNoisy) {}
    public int getCurrentBufferedPercentage() { return 0; }

    public boolean isCurrentMediaItemSeekable() { return false; }
    public boolean isCurrentWindowSeekable() { return false; }
}
