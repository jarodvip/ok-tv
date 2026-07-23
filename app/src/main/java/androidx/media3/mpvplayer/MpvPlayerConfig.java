package androidx.media3.mpvplayer;

public final class MpvPlayerConfig {
    public static final String VIDEO_OUTPUT_GPU_NEXT = "gpu-next";
    private final int decode;
    private final String defaultUserAgent;
    private final boolean hlsHttpPersistent;

    private MpvPlayerConfig(Builder builder) {
        this.decode = builder.decode;
        this.defaultUserAgent = builder.defaultUserAgent;
        this.hlsHttpPersistent = builder.hlsHttpPersistent;
    }

    public int getDecode() { return decode; }
    public String getDefaultUserAgent() { return defaultUserAgent; }
    public boolean isHlsHttpPersistent() { return hlsHttpPersistent; }

    public static final class Builder {
        private int decode;
        private String defaultUserAgent;
        private boolean hlsHttpPersistent;

        public Builder() {}

        public Builder setDecode(int decode) { this.decode = decode; return this; }
        public Builder setDefaultUserAgent(String defaultUserAgent) { this.defaultUserAgent = defaultUserAgent; return this; }
        public Builder setHlsHttpPersistent(boolean hlsHttpPersistent) { this.hlsHttpPersistent = hlsHttpPersistent; return this; }
        public Builder addConfigDirectory(String path) { return this; }
        public Builder addAndroidFontConfig(String configDir, String cacheDir) { return this; }
        public Builder addAndroidDefaults(String videoOutput, String cacheDir) { return this; }
        public Builder addTlsCaFileFromAsset(Object context, String asset, String dest) { return this; }
        public Builder addPostInitStringOption(String key, Object value) { return this; }
        public Builder addPreInitStringOption(String key, String value) { return this; }
        public Builder addDiskCacheOptions(String cacheDir, long seconds, int sizeMb) { return this; }
        public Builder addAndroidSubtitleOptions(Object context, boolean enabled, double position, double scale) { return this; }
        public MpvPlayerConfig build() { return new MpvPlayerConfig(this); }
    }
}
