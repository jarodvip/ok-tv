package com.fongmi.android.tv.setting;

import android.text.TextUtils;

import com.fongmi.android.tv.ui.danmaku.DanmakuConfig;

import com.fongmi.android.tv.api.config.VodConfig;
import com.github.catvod.utils.Prefers;

public class DanmakuSetting {

    private static final float MIN_TEXT_SCALE = 0.5f;
    private static final float MAX_TEXT_SCALE = 3.0f;
    private static final float MIN_TRANSPARENCY = 0.0f;
    private static final float MAX_TRANSPARENCY = 0.9f;
    private static final float MIN_STROKE_WIDTH_MULTIPLIER = 0.05f;
    private static final float MAX_STROKE_WIDTH_MULTIPLIER = 0.3f;
    private static final float MIN_PROJECTION_OFFSET = 0.02f;
    private static final float MAX_PROJECTION_OFFSET = 0.15f;
    private static final long MIN_TIME_OFFSET_MS = -300000L;
    private static final long MAX_TIME_OFFSET_MS = 300000L;
    private static final long MIN_DURATION_MS = 3000L;
    private static final long MAX_DURATION_MS = 15000L;
    private static final long MIN_FIXED_DURATION_MS = 2000L;
    private static final long MAX_FIXED_DURATION_MS = 10000L;
    private static final int MIN_MAX_ON_SCREEN = 10;
    private static final int MAX_MAX_ON_SCREEN = 500;
    private static final float MIN_SCROLL_AREA_RATIO = 0.1f;
    private static final float MAX_SCROLL_AREA_RATIO = 1.0f;
    private static final float MIN_SCROLL_GAP_RATIO = 0.0f;
    private static final float MAX_SCROLL_GAP_RATIO = 5.0f;
    private static final float MIN_LINE_SPACING = 1.0f;
    private static final float MAX_LINE_SPACING = 2.0f;
    private static final int MIN_MAX_SCROLL_LINES = 0;
    private static final int MAX_MAX_SCROLL_LINES = 20;
    private static final int MIN_MAX_FIXED_LINES = 0;
    private static final int MAX_MAX_FIXED_LINES = 10;

    public static boolean isLoad() {
        return Prefers.getBoolean("danmaku_load");
    }

    public static void putLoad(boolean danmakuLoad) {
        Prefers.put("danmaku_load", danmakuLoad);
        if (danmakuLoad) putShow(true);
    }

    public static boolean isAuto() {
        return Prefers.getBoolean("danmaku_auto");
    }

    public static void putAuto(boolean auto) {
        Prefers.put("danmaku_auto", auto);
    }

    public static boolean isSpiderFirst() {
        return Prefers.getBoolean("danmaku_spider_first");
    }

    public static void putSpiderFirst(boolean spiderFirst) {
        Prefers.put("danmaku_spider_first", spiderFirst);
    }

    public static String getApiUrl() {
        return Prefers.getString("danmaku_api_url", "");
    }

    public static void putApiUrl(String url) {
        Prefers.put("danmaku_api_url", url);
    }

    public static boolean isShow() {
        return Prefers.getBoolean("danmaku_show", true);
    }

    public static void putShow(boolean danmakuShow) {
        Prefers.put("danmaku_show", danmakuShow);
    }

    public static float getTextScale() {
        return Math.max(MIN_TEXT_SCALE, Math.min(MAX_TEXT_SCALE, Prefers.getFloat("danmaku_text_scale", 1f)));
    }

    public static void putTextScale(float value) {
        Prefers.put("danmaku_text_scale", Math.max(MIN_TEXT_SCALE, Math.min(MAX_TEXT_SCALE, value)));
    }

    public static float getTransparency() {
        return Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, Prefers.getFloat("danmaku_transparency", 0f)));
    }

    public static void putTransparency(float value) {
        Prefers.put("danmaku_transparency", Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, value)));
    }

    public static boolean isTextBold() {
        return Prefers.getBoolean("danmaku_text_bold");
    }

    public static void putTextBold(boolean value) {
        Prefers.put("danmaku_text_bold", value);
    }

    public static int getStyleMode() {
        return Prefers.getInt("danmaku_style_mode", DanmakuConfig.STYLE_STROKE);
    }

    public static void putStyleMode(int value) {
        Prefers.put("danmaku_style_mode", value);
    }

    public static int getColorMode() {
        return Prefers.getInt("danmaku_color_mode", DanmakuConfig.COLOR_MODE_DEFAULT);
    }

    public static void putColorMode(int value) {
        Prefers.put("danmaku_color_mode", value);
    }

    public static float getShadowTransparency() {
        return Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, Prefers.getFloat("danmaku_shadow_transparency", 0.1f)));
    }

    public static void putShadowTransparency(float value) {
        Prefers.put("danmaku_shadow_transparency", Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, value)));
    }

    public static float getStrokeWidthMultiplier() {
        return Math.max(MIN_STROKE_WIDTH_MULTIPLIER, Math.min(MAX_STROKE_WIDTH_MULTIPLIER, Prefers.getFloat("danmaku_stroke_width_multiplier", 0.12f)));
    }

    public static void putStrokeWidthMultiplier(float value) {
        Prefers.put("danmaku_stroke_width_multiplier", Math.max(MIN_STROKE_WIDTH_MULTIPLIER, Math.min(MAX_STROKE_WIDTH_MULTIPLIER, value)));
    }

    public static float getProjectionOffsetX() {
        return Math.max(MIN_PROJECTION_OFFSET, Math.min(MAX_PROJECTION_OFFSET, Prefers.getFloat("danmaku_projection_offset_x", 0.08f)));
    }

    public static void putProjectionOffsetX(float value) {
        Prefers.put("danmaku_projection_offset_x", Math.max(MIN_PROJECTION_OFFSET, Math.min(MAX_PROJECTION_OFFSET, value)));
    }

    public static float getProjectionOffsetY() {
        return Math.max(MIN_PROJECTION_OFFSET, Math.min(MAX_PROJECTION_OFFSET, Prefers.getFloat("danmaku_projection_offset_y", 0.08f)));
    }

    public static void putProjectionOffsetY(float value) {
        Prefers.put("danmaku_projection_offset_y", Math.max(MIN_PROJECTION_OFFSET, Math.min(MAX_PROJECTION_OFFSET, value)));
    }

    public static float getProjectionTransparency() {
        return Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, Prefers.getFloat("danmaku_projection_transparency", 0.2f)));
    }

    public static void putProjectionTransparency(float value) {
        Prefers.put("danmaku_projection_transparency", Math.max(MIN_TRANSPARENCY, Math.min(MAX_TRANSPARENCY, value)));
    }

    public static long getDurationMs() {
        return Math.max(MIN_DURATION_MS, Math.min(MAX_DURATION_MS, Prefers.getLong("danmaku_duration", 8000L)));
    }

    public static void putDurationMs(long value) {
        Prefers.put("danmaku_duration", Math.max(MIN_DURATION_MS, Math.min(MAX_DURATION_MS, value)));
    }

    public static long getFixedDurationMs() {
        return Math.max(MIN_FIXED_DURATION_MS, Math.min(MAX_FIXED_DURATION_MS, Prefers.getLong("danmaku_fixed_duration", 5000L)));
    }

    public static void putFixedDurationMs(long value) {
        Prefers.put("danmaku_fixed_duration", Math.max(MIN_FIXED_DURATION_MS, Math.min(MAX_FIXED_DURATION_MS, value)));
    }

    public static long getTimeOffsetMs() {
        return Math.max(MIN_TIME_OFFSET_MS, Math.min(MAX_TIME_OFFSET_MS, Prefers.getLong("danmaku_time_offset", 0L)));
    }

    public static void putTimeOffsetMs(long value) {
        Prefers.put("danmaku_time_offset", Math.max(MIN_TIME_OFFSET_MS, Math.min(MAX_TIME_OFFSET_MS, value)));
    }

    public static int getMaxOnScreen() {
        return Math.max(MIN_MAX_ON_SCREEN, Math.min(MAX_MAX_ON_SCREEN, Prefers.getInt("danmaku_max_on_screen", 150)));
    }

    public static void putMaxOnScreen(int value) {
        Prefers.put("danmaku_max_on_screen", Math.max(MIN_MAX_ON_SCREEN, Math.min(MAX_MAX_ON_SCREEN, value)));
    }

    public static float getScrollAreaRatio() {
        return Math.max(MIN_SCROLL_AREA_RATIO, Math.min(MAX_SCROLL_AREA_RATIO, Prefers.getFloat("danmaku_scroll_area_ratio", 0.5f)));
    }

    public static void putScrollAreaRatio(float value) {
        Prefers.put("danmaku_scroll_area_ratio", Math.max(MIN_SCROLL_AREA_RATIO, Math.min(MAX_SCROLL_AREA_RATIO, value)));
    }

    public static int getMaxScrollLines() {
        return Math.max(MIN_MAX_SCROLL_LINES, Math.min(MAX_MAX_SCROLL_LINES, Prefers.getInt("danmaku_max_scroll_lines", 0)));
    }

    public static void putMaxScrollLines(int value) {
        Prefers.put("danmaku_max_scroll_lines", Math.max(MIN_MAX_SCROLL_LINES, Math.min(MAX_MAX_SCROLL_LINES, value)));
    }

    public static int getMaxTopLines() {
        return Math.max(MIN_MAX_FIXED_LINES, Math.min(MAX_MAX_FIXED_LINES, Prefers.getInt("danmaku_max_top_lines", 0)));
    }

    public static void putMaxTopLines(int value) {
        Prefers.put("danmaku_max_top_lines", Math.max(MIN_MAX_FIXED_LINES, Math.min(MAX_MAX_FIXED_LINES, value)));
    }

    public static int getMaxBottomLines() {
        return Math.max(MIN_MAX_FIXED_LINES, Math.min(MAX_MAX_FIXED_LINES, Prefers.getInt("danmaku_max_bottom_lines", 0)));
    }

    public static void putMaxBottomLines(int value) {
        Prefers.put("danmaku_max_bottom_lines", Math.max(MIN_MAX_FIXED_LINES, Math.min(MAX_MAX_FIXED_LINES, value)));
    }

    public static float getLineSpacing() {
        return Math.max(MIN_LINE_SPACING, Math.min(MAX_LINE_SPACING, Prefers.getFloat("danmaku_line_spacing", 1.4f)));
    }

    public static void putLineSpacing(float value) {
        Prefers.put("danmaku_line_spacing", Math.max(MIN_LINE_SPACING, Math.min(MAX_LINE_SPACING, value)));
    }

    public static float getScrollGapRatio() {
        return Math.max(MIN_SCROLL_GAP_RATIO, Math.min(MAX_SCROLL_GAP_RATIO, Prefers.getFloat("danmaku_scroll_gap_ratio", 0f)));
    }

    public static void putScrollGapRatio(float value) {
        Prefers.put("danmaku_scroll_gap_ratio", Math.max(MIN_SCROLL_GAP_RATIO, Math.min(MAX_SCROLL_GAP_RATIO, value)));
    }

    public static boolean isShowScroll() {
        return Prefers.getBoolean("danmaku_show_scroll", true);
    }

    public static void putShowScroll(boolean value) {
        Prefers.put("danmaku_show_scroll", value);
    }

    public static boolean isShowTop() {
        return Prefers.getBoolean("danmaku_show_top", true);
    }

    public static void putShowTop(boolean value) {
        Prefers.put("danmaku_show_top", value);
    }

    public static boolean isShowBottom() {
        return Prefers.getBoolean("danmaku_show_bottom", true);
    }

    public static void putShowBottom(boolean value) {
        Prefers.put("danmaku_show_bottom", value);
    }

    public static boolean isShowReverse() {
        return Prefers.getBoolean("danmaku_show_reverse", true);
    }

    public static void putShowReverse(boolean value) {
        Prefers.put("danmaku_show_reverse", value);
    }

    public static boolean isShowPositioned() {
        return Prefers.getBoolean("danmaku_show_positioned", true);
    }

    public static void putShowPositioned(boolean value) {
        Prefers.put("danmaku_show_positioned", value);
    }

    public static boolean isShowSubtitle() {
        return Prefers.getBoolean("danmaku_show_subtitle", true);
    }

    public static void putShowSubtitle(boolean value) {
        Prefers.put("danmaku_show_subtitle", value);
    }

    public static boolean isShowSpecial() {
        return Prefers.getBoolean("danmaku_show_special", true);
    }

    public static void putShowSpecial(boolean value) {
        Prefers.put("danmaku_show_special", value);
    }

    public static String getEffectiveApiUrl() {
        String userUrl = getApiUrl();
        if (!TextUtils.isEmpty(userUrl)) return userUrl;
        return VodConfig.get().getConfig().getDanmaku();
    }

    public static void resetAppearance() {
        DanmakuConfig config = DanmakuConfig.DEFAULT;
        putTextScale(config.textScale);
        putTransparency(config.transparency);
        putTextBold(config.textBold);
        putStyleMode(config.styleMode);
        putShadowTransparency(config.shadowTransparency);
        putStrokeWidthMultiplier(config.strokeWidthMultiplier);
        putProjectionOffsetX(config.projectionOffsetXMultiplier);
        putProjectionOffsetY(config.projectionOffsetYMultiplier);
        putProjectionTransparency(config.projectionTransparency);
        putColorMode(config.colorMode);
    }

    public static void resetTiming() {
        DanmakuConfig config = DanmakuConfig.DEFAULT;
        putDurationMs(config.durationMs);
        putFixedDurationMs(config.fixedDurationMs);
        putTimeOffsetMs(config.timeOffsetMs);
    }

    public static void resetDensity() {
        DanmakuConfig config = DanmakuConfig.DEFAULT;
        putMaxOnScreen(config.maxOnScreen);
        putScrollAreaRatio(config.scrollAreaRatio);
        putScrollGapRatio(config.scrollGapRatio);
        putLineSpacing(config.lineSpacing);
        putMaxScrollLines(config.maxScrollLines);
        putMaxTopLines(config.maxTopLines);
        putMaxBottomLines(config.maxBottomLines);
    }

    public static void resetDisplay() {
        DanmakuConfig config = DanmakuConfig.DEFAULT;
        putShowScroll(config.showScroll);
        putShowTop(config.showTop);
        putShowBottom(config.showBottom);
        putShowReverse(config.showReverse);
        putShowPositioned(config.showPositioned);
        putShowSubtitle(config.showSubtitle);
        putShowSpecial(config.showSpecial);
    }

    public static DanmakuConfig getConfig() {
        return new DanmakuConfig.Builder()
                .setTextScale(getTextScale())
                .setTransparency(getTransparency())
                .setTextBold(isTextBold())
                .setStyleMode(getStyleMode())
                .setShadowTransparency(getShadowTransparency())
                .setStrokeWidthMultiplier(getStrokeWidthMultiplier())
                .setProjectionOffsetXMultiplier(getProjectionOffsetX())
                .setProjectionOffsetYMultiplier(getProjectionOffsetY())
                .setProjectionTransparency(getProjectionTransparency())
                .setColorMode(getColorMode())
                .setDurationMs(getDurationMs())
                .setFixedDurationMs(getFixedDurationMs())
                .setTimeOffsetMs(getTimeOffsetMs())
                .setMaxOnScreen(getMaxOnScreen())
                .setScrollAreaRatio(getScrollAreaRatio())
                .setScrollGapRatio(getScrollGapRatio())
                .setLineSpacing(getLineSpacing())
                .setMaxScrollLines(getMaxScrollLines())
                .setMaxTopLines(getMaxTopLines())
                .setMaxBottomLines(getMaxBottomLines())
                .setShowScroll(isShowScroll())
                .setShowTop(isShowTop())
                .setShowBottom(isShowBottom())
                .setShowReverse(isShowReverse())
                .setShowPositioned(isShowPositioned())
                .setShowSubtitle(isShowSubtitle())
                .setShowSpecial(isShowSpecial())
                .build();
    }
}
