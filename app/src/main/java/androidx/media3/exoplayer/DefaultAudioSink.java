package androidx.media3.exoplayer.audio;

import android.content.Context;

public class DefaultAudioSink {

    public static final class Builder {
        private final Context context;
        private boolean floatOutput;
        private boolean playbackParams;
        private AudioTrackAudioOutputProvider provider;

        public Builder(Context context) { this.context = context; }
        public Builder setEnableFloatOutput(boolean floatOutput) { this.floatOutput = floatOutput; return this; }
        public Builder setEnableAudioOutputPlaybackParameters(boolean playbackParams) { this.playbackParams = playbackParams; return this; }
        public Builder setAudioOutputProvider(AudioTrackAudioOutputProvider provider) { this.provider = provider; return this; }
        public DefaultAudioSink build() { return new DefaultAudioSink(); }
    }

    private DefaultAudioSink() {}
}
