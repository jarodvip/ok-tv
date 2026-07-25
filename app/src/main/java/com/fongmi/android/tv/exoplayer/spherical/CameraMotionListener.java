package com.fongmi.android.tv.exoplayer.spherical;

public interface CameraMotionListener {
    void onCameraMotion(long timestampUs, float[] rotation);
    void onCameraMotionReset();
}
