package androidx.media3.session;

import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import com.google.common.collect.ImmutableList;
import com.google.common.util.concurrent.ListenableFuture;

import java.util.List;

public class MediaLibraryService {

    public static final class LibraryParams {
        public final int extras;
        public final boolean supportsRecent;
        public final boolean supportsPlaylists;
        public final boolean supportsSearch;
        public final boolean supportsQueue;
        public final int maxPlaylistSize;
        public final int initialPageSize;

        private LibraryParams(Builder builder) {
            this.extras = builder.extras;
            this.supportsRecent = builder.supportsRecent;
            this.supportsPlaylists = builder.supportsPlaylists;
            this.supportsSearch = builder.supportsSearch;
            this.supportsQueue = builder.supportsQueue;
            this.maxPlaylistSize = builder.maxPlaylistSize;
            this.initialPageSize = builder.initialPageSize;
        }

        public static final class Builder {
            private int extras;
            private boolean supportsRecent;
            private boolean supportsPlaylists;
            private boolean supportsSearch;
            private boolean supportsQueue;
            private int maxPlaylistSize;
            private int initialPageSize;

            public Builder() {}
            public Builder setExtras(@Nullable Bundle extras) { this.extras = extras != null ? extras.getInt("stub", 0) : 0; return this; }
            public Builder setSupportsRecent(boolean supportsRecent) { this.supportsRecent = supportsRecent; return this; }
            public Builder setSupportsPlaylists(boolean supportsPlaylists) { this.supportsPlaylists = supportsPlaylists; return this; }
            public Builder setSupportsSearch(boolean supportsSearch) { this.supportsSearch = supportsSearch; return this; }
            public Builder setSupportsQueue(boolean supportsQueue) { this.supportsQueue = supportsQueue; return this; }
            public Builder setMaxPlaylistSize(int maxPlaylistSize) { this.maxPlaylistSize = maxPlaylistSize; return this; }
            public Builder setInitialPageSize(int initialPageSize) { this.initialPageSize = initialPageSize; return this; }
            public LibraryParams build() { return new LibraryParams(this); }
        }
    }

    public static final class MediaLibrarySession extends MediaSession {
        public void notifyChildrenChanged(@NonNull MediaSession.ControllerInfo controller, @NonNull String parentId, int itemCount, @Nullable LibraryParams params) {}
        public void notifyChildrenChanged(@NonNull String parentId, int itemCount, @Nullable LibraryParams params) {}
        public void notifySearchResultChanged(@NonNull MediaSession.ControllerInfo controller, @NonNull String query, int itemCount, @Nullable LibraryParams params) {}
        public ImmutableList<MediaSession.ControllerInfo> getSubscribedControllers(@NonNull String parentId) { return ImmutableList.of(); }

        public interface Callback extends MediaSession.Callback {
            ListenableFuture<LibraryResult<Void>> onGetLibraryRoot(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @Nullable LibraryParams params);
            ListenableFuture<LibraryResult<ImmutableList<androidx.media3.common.MediaItem>>> onGetChildren(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String parentId, int page, int pageSize, @Nullable LibraryParams params);
            ListenableFuture<LibraryResult<Void>> onSearch(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String query, @Nullable LibraryParams params);
            ListenableFuture<LibraryResult<ImmutableList<androidx.media3.common.MediaItem>>> onGetSearchResult(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String query, int page, int pageSize, @Nullable LibraryParams params);
            ListenableFuture<LibraryResult<androidx.media3.common.MediaItem>> onGetItem(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String mediaId);
            ListenableFuture<androidx.media3.common.MediaItem> onSubscribe(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String parentId, @Nullable LibraryParams params);
            void onUnsubscribe(@NonNull MediaLibrarySession session, @NonNull MediaSession.ControllerInfo browser, @NonNull String parentId);
        }

        public static final class Builder {
            public Builder(MediaLibraryService service, androidx.media3.common.Player player, Callback callback) {}
            public MediaLibrarySession build() { return new MediaLibrarySession(); }
        }

        public static final class ConnectionResult extends MediaSession.ConnectionResult {
            public static final class AcceptedResultBuilder {
                public AcceptedResultBuilder(MediaSession session) {}
                public AcceptedResultBuilder setAvailableSessionCommands(SessionCommands commands) { return this; }
                public AcceptedResultBuilder setAvailablePlayerCommands(androidx.media3.common.Player.Commands commands) { return this; }
                public AcceptedResultBuilder setCustomLayout(List<CommandButton> customLayout) { return this; }
                public AcceptedResultBuilder setSessionExtras(Bundle extras) { return this; }
                public ConnectionResult build() { return new ConnectionResult(); }
            }
        }
    }
}
