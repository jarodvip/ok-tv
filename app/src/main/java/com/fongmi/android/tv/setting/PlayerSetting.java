package com.fongmi.android.tv.setting;

import android.content.Intent;
import android.provider.Settings;

import com.fongmi.android.tv.App;
import com.github.catvod.utils.Prefers;

public class PlayerSetting {

    public static final int ENGINE_EXO = 0;
    public static final int ENGINE_MPV = 1;
    public static final int RENDER_SURFACE = 0;
    public static final int RENDER_TEXTURE = 1;
    public static final int MIN_SCALE = 0;
    public static final int MAX_SCALE = 4;
    private static final int MIN_SIZE = 0;
    private static final int MAX_SIZE = 3;
    private static final int MIN_BACKGROUND = 0;
    private static final int MAX_BACKGROUND = 2;
    private static final float MIN_SPEED = 2.0f;
    private static final float MAX_SPEED = 5.0f;

    public static int getEngine() {
        return Math.max(ENGINE_EXO, Math.min(ENGINE_MPV, Prefers.getInt("player_engine", ENGINE_EXO)));
    }

    public static void putEngine(int engine) {
        Prefers.put("player_engine", Math.max(ENGINE_EXO, Math.min(ENGINE_MPV, engine)));
        if (!isMpv() && isTunnel()) Prefers.put("render", RENDER_SURFACE);
    }

    public static boolean isMpv() {
        return getEngine() == ENGINE_MPV;
    }

    public static boolean isMpvGpuNext() {
        return Prefers.getBoolean("mpv_gpu_next");
    }

    public static void putMpvGpuNext(boolean gpuNext) {
        Prefers.put("mpv_gpu_next", gpuNext);
    }

    public static boolean isMpvVulkan() {
        return Prefers.getBoolean("mpv_vulkan");
    }

    public static void putMpvVulkan(boolean vulkan) {
        Prefers.put("mpv_vulkan", vulkan);
    }

    public static int getRender() {
        return Math.max(RENDER_SURFACE, Math.min(RENDER_TEXTURE, Prefers.getInt("render", RENDER_SURFACE)));
    }

    public static void putRender(int render) {
        Prefers.put("render", Math.max(RENDER_SURFACE, Math.min(RENDER_TEXTURE, render)));
        if (!isMpv() && isTunnel() && getRender() == RENDER_TEXTURE) Prefers.put("tunnel", false);
    }

    public static boolean isTunnel() {
        return Prefers.getBoolean("tunnel");
    }

    public static void putTunnel(boolean tunnel) {
        Prefers.put("tunnel", tunnel);
        if (!isMpv() && tunnel) Prefers.put("render", RENDER_SURFACE);
    }

    public static boolean isTunnelingEnabled() {
        return isTunnel() && getRender() == RENDER_SURFACE;
    }

    public static int getSize() {
        return Math.max(MIN_SIZE, Math.min(MAX_SIZE, Prefers.getInt("size", 2)));
    }

    public static void putSize(int size) {
        Prefers.put("size", Math.max(MIN_SIZE, Math.min(MAX_SIZE, size)));
    }

    public static int getScale() {
        return Math.max(MIN_SCALE, Math.min(MAX_SCALE, Prefers.getInt("scale")));
    }

    public static void putScale(int scale) {
        Prefers.put("scale", Math.max(MIN_SCALE, Math.min(MAX_SCALE, scale)));
    }

    public static int getBackground() {
        return Math.max(MIN_BACKGROUND, Math.min(MAX_BACKGROUND, Prefers.getInt("background", 2)));
    }

    public static void putBackground(int background) {
        Prefers.put("background", Math.max(MIN_BACKGROUND, Math.min(MAX_BACKGROUND, background)));
    }

    public static boolean isBackgroundOff() {
        return getBackground() == 0;
    }

    public static boolean isBackgroundOn() {
        return getBackground() == 1 || getBackground() == 2;
    }

    public static boolean isBackgroundPiP() {
        return getBackground() == 2;
    }

    public static float getSpeed() {
        return Math.max(MIN_SPEED, Math.min(MAX_SPEED, Prefers.getFloat("speed", 3)));
    }

    public static void putSpeed(float speed) {
        Prefers.put("speed", Math.max(MIN_SPEED, Math.min(MAX_SPEED, speed)));
    }

    public static boolean isCaption() {
        return Prefers.getBoolean("caption");
    }

    public static void putCaption(boolean caption) {
        Prefers.put("caption", caption);
    }

    public static float getSubtitleTextSize() {
        return Prefers.getFloat("subtitle_text_size");
    }

    public static void putSubtitleTextSize(float value) {
        Prefers.put("subtitle_text_size", value);
    }

    public static float getSubtitlePosition() {
        return Prefers.getFloat("subtitle_position");
    }

    public static void putSubtitlePosition(float value) {
        Prefers.put("subtitle_position", value);
    }

    public static boolean hasCaption() {
        return new Intent(Settings.ACTION_CAPTIONING_SETTINGS).resolveActivity(App.get().getPackageManager()) != null;
    }

    public static boolean isAudioPassThrough() {
        return Prefers.getBoolean("audio_pass_through", true);
    }

    public static void putAudioPassThrough(boolean audioPassThrough) {
        Prefers.put("audio_pass_through", audioPassThrough);
    }

    public static boolean isAudioPrefer() {
        return Prefers.getBoolean("audio_prefer");
    }

    public static void putAudioPrefer(boolean audioPrefer) {
        Prefers.put("audio_prefer", audioPrefer);
    }

    public static boolean isVideoPrefer() {
        return Prefers.getBoolean("video_prefer");
    }

    public static void putVideoPrefer(boolean videoPrefer) {
        Prefers.put("video_prefer", videoPrefer);
    }

    public static boolean isPreferAAC() {
        return Prefers.getBoolean("prefer_aac");
    }

    public static void putPreferAAC(boolean preferAAC) {
        Prefers.put("prefer_aac", preferAAC);
    }

    public static boolean isDv7HevcFallback() {
        return Prefers.getBoolean("dv7_hevc_fallback");
    }

    public static void putDv7HevcFallback(boolean fallback) {
        Prefers.put("dv7_hevc_fallback", fallback);
    }
}
