package com.fongmi.android.tv.mpvplayer;

import android.content.Context;
import android.widget.Toast;

import com.fongmi.android.tv.App;

public class MpvPlayerConfig {

    public static final String VIDEO_OUTPUT_GPU_NEXT = "gpu-next";

    public static final class Builder {
        public Builder() {}
        public Builder setDefaultUserAgent(String ua) { return this; }
        public Builder setHlsHttpPersistent(boolean persistent) { return this; }
        public Builder addConfigDirectory(String dir) { return this; }
        public Builder addAndroidFontConfig(String configDir, String cacheDir) { return this; }
        public Builder addAndroidDefaults(String videoOutput, String cacheDir) { return this; }
        public Builder addTlsCaFileFromAsset(Context context, String assetPath, String destPath) { return this; }
        public Builder addPostInitStringOption(String option, Object value) { return this; }
        public Builder addPreInitStringOption(String option, String value) { return this; }
        public Builder addDiskCacheOptions(String dir, int time, int size) { return this; }
        public Builder addAndroidSubtitleOptions(Context context, boolean caption, double position, double scale) { return this; }
        public MpvPlayerConfig build() { return new MpvPlayerConfig(); }
    }
}
