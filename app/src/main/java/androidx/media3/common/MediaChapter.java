package androidx.media3.common;

public final class MediaChapter {
    public static MediaChapter DEFAULT = new MediaChapter("", C.TIME_UNSET);
    public final String label;
    public final long timeUs;

    public MediaChapter(String label, long timeUs) {
        this.label = label;
        this.timeUs = timeUs;
    }
}
