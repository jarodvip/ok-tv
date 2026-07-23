package androidx.media3.session;

import android.app.PendingIntent;
import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import com.google.common.collect.ImmutableList;
import com.google.common.util.concurrent.Futures;
import com.google.common.util.concurrent.ListenableFuture;

import java.util.List;

public class MediaSession {

    public final PendingIntent sessionActivity;
    public final String id;
    public final androidx.media3.common.Player player;

    public MediaSession() {
        this.sessionActivity = null;
        this.id = null;
        this.player = null;
    }

    public PendingIntent getSessionActivity() { return sessionActivity; }
    public void setSessionActivity(PendingIntent pendingIntent) {}
    public void setPlayer(@NonNull androidx.media3.common.Player player) {}
    public void release() {}
    public androidx.media3.common.Player getPlayer() { return player; }
    public String getId() { return id; }
    public MediaSessionToken getToken() { return null; }
    public List<ControllerInfo> getConnectedControllers() { return ImmutableList.of(); }
    public ControllerInfo getControllerForCurrentRequest() { return null; }
    public boolean isMediaNotificationController(@NonNull ControllerInfo controller) { return false; }
    public ControllerInfo getMediaNotificationControllerInfo() { return null; }
    public boolean isAutomotiveController(@NonNull ControllerInfo controller) { return false; }
    public boolean isAutoCompanionController(@NonNull ControllerInfo controller) { return false; }
    public ListenableFuture<SessionResult> setCustomLayout(@NonNull ControllerInfo controller, List<CommandButton> commandButtons) { return Futures.immediateFuture(new SessionResult(SessionResult.RESULT_SUCCESS)); }
    public void setCustomLayout(List<CommandButton> commandButtons) {}
    public void setAvailableCommands(@NonNull ControllerInfo controller, SessionCommands sessionCommands, androidx.media3.common.Player.Commands playerCommands) {}
    public ImmutableList<CommandButton> getCustomLayout() { return ImmutableList.of(); }
    public void broadcastCustomCommand(@NonNull SessionCommand sessionCommand, Bundle args) {}
    public Bundle getSessionExtras() { return Bundle.EMPTY; }
    public void setSessionExtras(Bundle extras) {}
    public void setSessionExtras(@NonNull ControllerInfo controller, Bundle extras) {}
    public void setMediaButtonPreferences(ImmutableList<CommandButton> commandButtons) {}
    public ImmutableList<CommandButton> getMediaButtonPreferences() { return ImmutableList.of(); }
    public ListenableFuture<SessionResult> sendCustomCommand(@NonNull ControllerInfo controller, @NonNull SessionCommand command, Bundle args) { return Futures.immediateFuture(new SessionResult(SessionResult.RESULT_SUCCESS)); }
    public android.support.v4.media.session.MediaSessionCompat.Token getSessionCompatToken() { return null; }

    public static final class ControllerInfo {
        public final String packageName;
        public ControllerInfo(String packageName) { this.packageName = packageName; }
    }

    public static class ConnectionResult {
        public static final SessionCommands DEFAULT_SESSION_COMMANDS = SessionCommands.EMPTY;
        public static final SessionCommands DEFAULT_SESSION_AND_LIBRARY_COMMANDS = SessionCommands.EMPTY;
        public static final androidx.media3.common.Player.Commands DEFAULT_PLAYER_COMMANDS = new androidx.media3.common.Player.Commands.Builder().build();
        public final boolean isAccepted;
        public final SessionCommands availableSessionCommands;
        public final androidx.media3.common.Player.Commands availablePlayerCommands;
        public final ImmutableList<CommandButton> customLayout;
        public final Bundle sessionExtras;

        public static ConnectionResult accept(SessionCommands sessionCommands, androidx.media3.common.Player.Commands playerCommands) { return new ConnectionResult(); }
        public static ConnectionResult reject() { return new ConnectionResult(); }

        public static class AcceptedResultBuilder {
            private SessionCommands commands;

            public AcceptedResultBuilder(MediaSession session) {}
            public AcceptedResultBuilder setAvailableSessionCommands(SessionCommands commands) { this.commands = commands; return this; }
            public AcceptedResultBuilder setAvailablePlayerCommands(androidx.media3.common.Player.Commands commands) { return this; }
            public AcceptedResultBuilder setCustomLayout(List<CommandButton> customLayout) { return this; }
            public AcceptedResultBuilder setSessionExtras(Bundle extras) { return this; }
            public ConnectionResult build() { return new ConnectionResult(); }
        }
    }

    public static final class MediaItemsWithStartPosition {
        public final List<androidx.media3.common.MediaItem> mediaItems;
        public final int startIndex;
        public final long startPositionMs;
        public MediaItemsWithStartPosition(List<androidx.media3.common.MediaItem> mediaItems, int startIndex, long startPositionMs) {
            this.mediaItems = mediaItems;
            this.startIndex = startIndex;
            this.startPositionMs = startPositionMs;
        }
    }

    public interface Callback {
        default ConnectionResult onConnect(MediaSession session, ControllerInfo controller) { return null; }
        default void onPostConnect(MediaSession session, ControllerInfo controller) {}
        default void onDisconnected(MediaSession session, ControllerInfo controller) {}
        default int onPlayerCommandRequest(MediaSession session, ControllerInfo controller, int command) { return 0; }
        default ListenableFuture<SessionResult> onSetRating(MediaSession session, ControllerInfo controller, String rating) { return Futures.immediateFuture(new SessionResult(SessionResult.RESULT_SUCCESS)); }
        default ListenableFuture<SessionResult> onSetRating(MediaSession session, ControllerInfo controller, androidx.media3.common.Rating rating) { return Futures.immediateFuture(new SessionResult(SessionResult.RESULT_SUCCESS)); }
        default ListenableFuture<SessionResult> onCustomCommand(MediaSession session, ControllerInfo controller, SessionCommand customCommand, Bundle args) { return Futures.immediateFuture(new SessionResult(SessionResult.RESULT_SUCCESS)); }
        default ListenableFuture<List<androidx.media3.common.MediaItem>> onAddMediaItems(MediaSession session, ControllerInfo controller, List<androidx.media3.common.MediaItem> mediaItems) { return Futures.immediateFuture(ImmutableList.of()); }
        default ListenableFuture<MediaItemsWithStartPosition> onSetMediaItems(MediaSession session, ControllerInfo controller, List<androidx.media3.common.MediaItem> mediaItems, int startIndex, long startPositionMs) { return null; }
        default ListenableFuture<MediaItemsWithStartPosition> onPlaybackResumption(MediaSession session, ControllerInfo controller) { return null; }
        default boolean onMediaButtonEvent(MediaSession session, ControllerInfo controller, android.content.Intent intent) { return false; }
    }

    public static final class Builder {
        public Builder(MediaLibraryService service, androidx.media3.common.Player player, Callback callback) {}
        public MediaSession build() { return new MediaSession(); }
    }

    public static final class BuilderBase {
        public BuilderBase(MediaLibraryService service) {}
    }
}
