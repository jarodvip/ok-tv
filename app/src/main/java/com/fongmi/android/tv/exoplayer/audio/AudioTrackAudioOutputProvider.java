package androidx.media3.exoplayer.audio;

public final class AudioTrackAudioOutputProvider {

    public static final class Builder {
        public Builder() {}
        public Builder setAudioAttributes(androidx.media3.common.AudioAttributes attributes) { return this; }
        public AudioTrackAudioOutputProvider build() { return new AudioTrackAudioOutputProvider(); }
    }

    public AudioTrackAudioOutputProvider() {}
}
