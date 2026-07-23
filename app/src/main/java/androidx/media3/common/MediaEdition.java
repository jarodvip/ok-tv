package androidx.media3.common;

public final class MediaEdition {
    public static MediaEdition DEFAULT = new MediaEdition("", C.TIME_UNSET);
    public final String label;
    public final long durationUs;

    public MediaEdition(String label, long durationUs) {
        this.label = label;
        this.durationUs = durationUs;
    }
}
