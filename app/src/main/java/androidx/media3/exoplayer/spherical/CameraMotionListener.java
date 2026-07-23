package androidx.media3.exoplayer.spherical;

public interface CameraMotionListener {
    void onCameraMotion(long timestampUs, float[] rotation);
    void onCameraMotionReset();
}
