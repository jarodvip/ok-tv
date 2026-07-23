package androidx.media3.common;

import android.net.Uri;

public final class MediaMetadata {

    public static final MediaMetadata EMPTY = new MediaMetadata.Builder().build();

    public static final class Builder {
        public Builder() {}
        public Builder setTitle(CharSequence title) { return this; }
        public Builder setArtist(CharSequence artist) { return this; }
        public Builder setArtworkUri(Uri uri) { return this; }
        public MediaMetadata build() { return new MediaMetadata(); }
    }

    private MediaMetadata() {}
}
