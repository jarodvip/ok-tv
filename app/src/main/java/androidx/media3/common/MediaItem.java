package androidx.media3.common;

import android.net.Uri;
import android.os.Bundle;

import java.util.List;
import java.util.Map;

public final class MediaItem {

    public static final String DEFAULT_MEDIA_ID = "";
    public static final MediaItem EMPTY = new MediaItem.Builder().build();
    public final String mediaId;
    public final LocalConfiguration localConfiguration;
    public final LocalConfiguration playbackProperties;
    public final LiveConfiguration liveConfiguration;
    public final MediaMetadata mediaMetadata;
    public final ClippingConfiguration clippingConfiguration;
    public final ClippingProperties clippingProperties;
    public final RequestMetadata requestMetadata;

    public static MediaItem fromUri(Uri uri) {
        return new MediaItem.Builder().setUri(uri).build();
    }

    public static MediaItem fromUri(String uri) {
        return new MediaItem.Builder().setUri(uri).build();
    }

    public MediaItem.Builder buildUpon() { return new MediaItem.Builder(); }

    public boolean equals(Object obj) { return this == obj; }
    public int hashCode() { return System.identityHashCode(this); }
    public android.os.Bundle toBundle() { return Bundle.EMPTY; }
    public android.os.Bundle toBundleIncludeLocalConfiguration() { return Bundle.EMPTY; }
    public static MediaItem fromBundle(android.os.Bundle bundle) { return EMPTY; }

    private MediaItem( Builder builder) {
        this.mediaId = builder.mediaId;
        this.localConfiguration = builder.localConfiguration;
        this.playbackProperties = builder.playbackProperties;
        this.liveConfiguration = builder.liveConfiguration;
        this.mediaMetadata = builder.mediaMetadata;
        this.clippingConfiguration = builder.clippingConfiguration;
        this.clippingProperties = builder.clippingProperties;
        this.requestMetadata = builder.requestMetadata;
    }

    public static final class Builder {
        private String mediaId;
        private LocalConfiguration localConfiguration;
        private LocalConfiguration playbackProperties;
        private LiveConfiguration liveConfiguration;
        private MediaMetadata mediaMetadata;
        private ClippingConfiguration clippingConfiguration;
        private ClippingProperties clippingProperties;
        private RequestMetadata requestMetadata;

        public Builder() {}
        public Builder setMediaId(String mediaId) { this.mediaId = mediaId; return this; }
        public Builder setUri(Uri uri) { return this; }
        public Builder setUri(String uri) { return this; }
        public Builder setMimeType(String mimeType) { return this; }
        public Builder setSubtitleConfigurations(List<SubtitleConfiguration> subtitleConfigurations) { return this; }
        public Builder setDrmConfiguration(DrmConfiguration drmConfiguration) { return this; }
        public Builder setRequestMetadata(RequestMetadata requestMetadata) { this.requestMetadata = requestMetadata; return this; }
        public Builder setMediaMetadata(MediaMetadata mediaMetadata) { this.mediaMetadata = mediaMetadata; return this; }
        public Builder setImageDurationMs(long imageDurationMs) { return this; }
        public Builder setTag(Object tag) { return this; }
        public Builder setAdblock(boolean ignored) { return this; }
        public Builder setDecode(int ignored) { return this; }
        public MediaItem build() { return new MediaItem(this); }
    }

    public static final class RequestMetadata {
        public final Uri mediaUri;
        public final Bundle extras;

        private RequestMetadata(Uri mediaUri, Bundle extras) {
            this.mediaUri = mediaUri;
            this.extras = extras;
        }

        public static final class Builder {
            private Uri mediaUri;
            private Bundle extras;

            public Builder() {}
            public Builder setMediaUri(Uri mediaUri) { this.mediaUri = mediaUri; return this; }
            public Builder setExtras(Bundle extras) { this.extras = extras; return this; }
            public RequestMetadata build() { return new RequestMetadata(mediaUri, extras); }
        }
    }

    public static final class SubtitleConfiguration {
        public final Uri uri;
        public final String label;
        public final String mimeType;
        public final int selectionFlags;
        public final String language;

        private SubtitleConfiguration(Uri uri, String label, String mimeType, int selectionFlags, String language) {
            this.uri = uri;
            this.label = label;
            this.mimeType = mimeType;
            this.selectionFlags = selectionFlags;
            this.language = language;
        }

        public static final class Builder {
            private final Uri uri;
            private String label;
            private String mimeType;
            private int selectionFlags;
            private String language;

            public Builder(Uri uri) { this.uri = uri; }
            public Builder setLabel(String label) { this.label = label; return this; }
            public Builder setMimeType(String mimeType) { this.mimeType = mimeType; return this; }
            public Builder setSelectionFlags(int selectionFlags) { this.selectionFlags = selectionFlags; return this; }
            public Builder setLanguage(String language) { this.language = language; return this; }
            public SubtitleConfiguration build() { return new SubtitleConfiguration(uri, label, mimeType, selectionFlags, language); }
        }
    }

    public static final class DrmConfiguration {
        public final java.util.UUID uuid;

        private DrmConfiguration(java.util.UUID uuid) { this.uuid = uuid; }

        public static final class Builder {
            private final java.util.UUID uuid;
            private boolean multiSession;
            private boolean forceDefaultLicenseUri;
            private String licenseUri;
            private Map<String, String> licenseRequestHeaders;

            public Builder(Uri uuid) { this.uuid = uuid; }
            public Builder setMultiSession(boolean multiSession) { this.multiSession = multiSession; return this; }
            public Builder setForceDefaultLicenseUri(boolean forceDefaultLicenseUri) { this.forceDefaultLicenseUri = forceDefaultLicenseUri; return this; }
            public Builder setLicenseUri(String licenseUri) { this.licenseUri = licenseUri; return this; }
            public Builder setLicenseRequestHeaders(Map<String, String> licenseRequestHeaders) { this.licenseRequestHeaders = licenseRequestHeaders; return this; }
            public DrmConfiguration build() { return new DrmConfiguration(uuid); }
        }
    }

    public static final class LocalConfiguration {
        public final Uri uri;
        public final String mimeType;
        public final DrmConfiguration drmConfiguration;
        public final AdsConfiguration adsConfiguration;
        public final List<StreamKey> streamKeys;
        public final String customCacheKey;
        public final List<SubtitleConfiguration> subtitleConfigurations;
        public final List<Sub> subtitles;
        public final Object tag;
        public final long imageDurationMs;
    }

    public static final class AdsConfiguration {
        public final Object adTagUri;
    }

    public static final class Sub {
    }

    public static final class ClippingConfiguration {
        public final long startPositionMs;
        public final long endPositionMs;
    }

    public static final class ClippingProperties {
        public final long startPositionMs;
        public final long endPositionMs;
    }

    public static final class LiveConfiguration {
        public final long targetOffsetMs;
    }
}
