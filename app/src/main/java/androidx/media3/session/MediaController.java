package androidx.media3.session;

import android.content.ComponentName;
import android.content.Context;
import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import com.google.common.util.concurrent.Futures;
import com.google.common.util.concurrent.ListenableFuture;

import java.util.List;
import java.util.concurrent.Executor;

public class MediaController implements androidx.media3.common.Player {

    public MediaController(@NonNull Context context, @NonNull SessionToken token) {}
    public MediaController(@NonNull Context context, @NonNull SessionToken token, @NonNull Executor executor) {}

    public static final class Builder {
        private final Context context;
        private SessionToken token;

        public Builder(@NonNull Context context, @NonNull SessionToken token) {
            this.context = context;
            this.token = token;
        }

        @NonNull
        public Builder setListener(@Nullable Listener listener) {
            return this;
        }

        @NonNull
        public MediaController build() {
            return new MediaController(context, token);
        }

        @NonNull
        public ListenableFuture<MediaController> buildAsync() {
            return Futures.immediateFuture(build());
        }
    }

    public static void releaseFuture(@Nullable ListenableFuture<? extends MediaController> future) {
        if (future != null && !future.isDone()) future.cancel(false);
    }

    public void addListener(@Nullable Listener listener) {}
    public void removeListener(@Nullable Listener listener) {}

    public interface Listener {
        default void onConnected(@NonNull MediaController controller, @Nullable Bundle extras) {}
        default void onDisconnected(@NonNull MediaController controller) {}
    }

    @Override
    public void release() {}
    @Override
    public int getPlaybackState() {
        return androidx.media3.common.Player.STATE_IDLE;
    }
    @Override
    public boolean isPlaying() {
        return false;
    }
    @Override
    public void play() {}
    @Override
    public void pause() {}
    @Override
    public void stop() {}
    @Override
    public long getDuration() {
        return androidx.media3.common.C.TIME_UNSET;
    }
    @Override
    public long getCurrentPosition() {
        return 0;
    }
    @Override
    public long getBufferedPosition() {
        return 0;
    }
    @Override
    public int getBufferedPercentage() {
        return 0;
    }
    @Override
    public long getTotalBufferedDuration() {
        return 0;
    }
    @Override
    public boolean isCurrentWindowDynamic() {
        return false;
    }
    @Override
    public boolean isCurrentMediaItemDynamic() {
        return false;
    }
    @Override
    public boolean isCurrentWindowLive() {
        return false;
    }
    @Override
    public boolean isCurrentMediaItemLive() {
        return false;
    }
    @Override
    public long getCurrentLiveOffset() {
        return 0;
    }
    @Override
    public boolean isCurrentWindowSeekable() {
        return false;
    }
    @Override
    public boolean isCurrentMediaItemSeekable() {
        return false;
    }
    @Override
    public boolean isPlayingAd() {
        return false;
    }
    @Override
    public int getCurrentAdGroupIndex() {
        return 0;
    }
    @Override
    public int getCurrentAdIndexInAdGroup() {
        return 0;
    }
    @Override
    public long getContentDuration() {
        return 0;
    }
    @Override
    public long getContentPosition() {
        return 0;
    }
    @Override
    public long getContentBufferedPosition() {
        return 0;
    }
    @Override
    public androidx.media3.common.AudioAttributes getAudioAttributes() {
        return null;
    }
    @Override
    public void setVolume(float volume) {}
    @Override
    public float getVolume() {
        return 0;
    }
    @Override
    public void clearVideoSurface() {}
    @Override
    public void clearVideoSurface(android.view.Surface surface) {}
    @Override
    public void setVideoSurface(android.view.Surface surface) {}
    @Override
    public void setVideoSurfaceHolder(android.view.SurfaceHolder holder) {}
    @Override
    public void clearVideoSurfaceHolder(android.view.SurfaceHolder holder) {}
    @Override
    public void setVideoSurfaceView(android.view.SurfaceView view) {}
    @Override
    public void clearVideoSurfaceView(android.view.SurfaceView view) {}
    @Override
    public void setVideoTextureView(android.view.TextureView view) {}
    @Override
    public void clearVideoTextureView(android.view.TextureView view) {}
    @Override
    public androidx.media3.common.VideoSize getVideoSize() {
        return null;
    }
    @Override
    public androidx.media3.common.util.Size getSurfaceSize() {
        return null;
    }
    @Override
    public androidx.media3.common.text.CueGroup getCurrentCues() {
        return new androidx.media3.common.text.CueGroup();
    }
    @Override
    public androidx.media3.common.DeviceInfo getDeviceInfo() {
        return null;
    }
    @Override
    public int getDeviceVolume() {
        return 0;
    }
    @Override
    public boolean isDeviceMuted() {
        return false;
    }
    @Override
    public void setDeviceVolume(int volume) {}
    @Override
    public void setDeviceVolume(int volume, int flags) {}
    @Override
    public void increaseDeviceVolume() {}
    @Override
    public void increaseDeviceVolume(int flags) {}
    @Override
    public void decreaseDeviceVolume() {}
    @Override
    public void decreaseDeviceVolume(int flags) {}
    @Override
    public void setDeviceMuted(boolean muted) {}
    @Override
    public void setDeviceMuted(boolean muted, int flags) {}
    @Override
    public void setAudioAttributes(androidx.media3.common.AudioAttributes attributes, boolean handleAudioFocus) {}
    @Override
    public void setMediaItems(List<androidx.media3.common.MediaItem> mediaItems) {}
    @Override
    public void setMediaItems(List<androidx.media3.common.MediaItem> mediaItems, boolean resetPosition) {}
    @Override
    public void setMediaItems(List<androidx.media3.common.MediaItem> mediaItems, int startIndex, long startPositionMs) {}
    @Override
    public void addMediaItems(List<androidx.media3.common.MediaItem> mediaItems) {}
    @Override
    public void addMediaItem(androidx.media3.common.MediaItem mediaItem) {}
    @Override
    public void removeMediaItems(List<androidx.media3.common.MediaItem> mediaItems) {}
    @Override
    public void removeMediaItem(int index) {}
    @Override
    public void clearMediaItems() {}
    @Override
    public boolean isCommandAvailable(int command) {
        return false;
    }
    @Override
    public boolean canAdvertiseSession() {
        return false;
    }
    @Override
    public int getCurrentMediaItemIndex() {
        return 0;
    }
    @Override
    public int getMediaItemCount() {
        return 0;
    }
    @Override
    public androidx.media3.common.MediaItem getCurrentMediaItem() {
        return null;
    }
    @Override
    public androidx.media3.common.MediaItem getMediaItemAt(int index) {
        return null;
    }
    @Override
    public androidx.media3.common.Timeline getCurrentTimeline() {
        return null;
    }
    @Override
    public int getCurrentPeriodIndex() {
        return 0;
    }
    @Override
    public int getCurrentWindowIndex() {
        return 0;
    }
    @Override
    public int getNextWindowIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getNextMediaItemIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getPreviousWindowIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getPreviousMediaItemIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public void seekTo(long positionMs) {}
    @Override
    public void seekTo(int mediaItemIndex, long positionMs) {}
    @Override
    public void seekToDefaultPosition() {}
    @Override
    public void seekToDefaultPosition(int mediaItemIndex) {}
    @Override
    public void seekBack() {}
    @Override
    public void seekForward() {}
    @Override
    public void seekToPrevious() {}
    @Override
    public void seekToPreviousMediaItem() {}
    @Override
    public void seekToPreviousWindow() {}
    @Override
    public void seekToNext() {}
    @Override
    public void seekToNextMediaItem() {}
    @Override
    public void seekToNextWindow() {}
    @Override
    public void play() {}
    @Override
    public void pause() {}
    @Override
    public void prepare() {}
    @Override
    public void stop() {}
    @Override
    public void release() {}
    @Override
    public void setPlayWhenReady(boolean playWhenReady) {}
    @Override
    public boolean getPlayWhenReady() {
        return false;
    }
    @Override
    public int getPlaybackState() {
        return androidx.media3.common.Player.STATE_IDLE;
    }
    @Override
    public androidx.media3.common.PlaybackParameters getPlaybackParameters() {
        return null;
    }
    @Override
    public void setPlaybackParameters(androidx.media3.common.PlaybackParameters parameters) {}
    @Override
    public void setPlaybackSpeed(float speed) {}
    @Override
    public void setRepeatMode(int repeatMode) {}
    @Override
    public int getRepeatMode() {
        return androidx.media3.common.Player.REPEAT_MODE_OFF;
    }
    @Override
    public void setShuffleModeEnabled(boolean shuffleModeEnabled) {}
    @Override
    public boolean getShuffleModeEnabled() {
        return false;
    }
    @Override
    public boolean isPlayingAd() {
        return false;
    }
    @Override
    public androidx.media3.common.Tracks getCurrentTracks() {
        return null;
    }
    @Override
    public androidx.media3.common.TrackSelectionParameters getTrackSelectionParameters() {
        return null;
    }
    @Override
    public void setTrackSelectionParameters(androidx.media3.common.TrackSelectionParameters parameters) {}
    @Override
    public androidx.media3.common.MediaMetadata getMediaMetadata() {
        return null;
    }
    @Override
    public androidx.media3.common.MediaMetadata getPlaylistMetadata() {
        return null;
    }
    @Override
    public void setPlaylistMetadata(androidx.media3.common.MediaMetadata mediaMetadata) {}
    @Override
    public java.lang.Object getCurrentManifest() {
        return null;
    }
    @Override
    public androidx.media3.common.Timeline getCurrentTimeline() {
        return null;
    }
    @Override
    public int getCurrentPeriodIndex() {
        return 0;
    }
    @Override
    public int getCurrentWindowIndex() {
        return 0;
    }
    @Override
    public int getCurrentMediaItemIndex() {
        return 0;
    }
    @Override
    public int getNextWindowIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getNextMediaItemIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getPreviousWindowIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public int getPreviousMediaItemIndex() {
        return androidx.media3.common.C.INDEX_UNSET;
    }
    @Override
    public boolean hasNext() {
        return false;
    }
    @Override
    public boolean hasNextWindow() {
        return false;
    }
    @Override
    public boolean hasNextMediaItem() {
        return false;
    }
    @Override
    public void next() {}
    @Override
    public void seekToNextWindow() {}
    @Override
    public void seekToNextMediaItem() {}
    @Override
    public void seekToNext() {}
    @Override
    public void setShuffleModeEnabled(boolean shuffleModeEnabled) {}
    @Override
    public void setRepeatMode(int repeatMode) {}
    @Override
    public int getRepeatMode() {
        return androidx.media3.common.Player.REPEAT_MODE_OFF;
    }
    @Override
    public androidx.media3.common.Events events() {
        return null;
    }
}
