package androidx.media3.ui.danmaku;

public final class DanmakuConfig {
    public static final int STYLE_NONE = 0;
    public static final int STYLE_SHADOW = 1;
    public static final int STYLE_STROKE = 2;
    public static final int STYLE_PROJECTION = 3;
    public static final int COLOR_MODE_DEFAULT = 0;
    public static final int COLOR_MODE_COLORFUL = 1;
    public static final int COLOR_MODE_GRADIENT = 2;
    public static final DanmakuConfig DEFAULT = new Builder().build();

    public final float textScale;
    public final float transparency;
    public final boolean textBold;
    public final int styleMode;
    public final float shadowTransparency;
    public final float strokeWidthMultiplier;
    public final float projectionOffsetXMultiplier;
    public final float projectionOffsetYMultiplier;
    public final float projectionTransparency;
    public final int colorMode;
    public final long durationMs;
    public final long fixedDurationMs;
    public final long timeOffsetMs;
    public final int maxOnScreen;
    public final float scrollAreaRatio;
    public final float scrollGapRatio;
    public final float lineSpacing;
    public final int maxScrollLines;
    public final int maxTopLines;
    public final int maxBottomLines;
    public final boolean showScroll;
    public final boolean showTop;
    public final boolean showBottom;
    public final boolean showReverse;
    public final boolean showPositioned;
    public final boolean showSubtitle;
    public final boolean showSpecial;

    private DanmakuConfig(Builder builder) {
        this.textScale = builder.textScale;
        this.transparency = builder.transparency;
        this.textBold = builder.textBold;
        this.styleMode = builder.styleMode;
        this.shadowTransparency = builder.shadowTransparency;
        this.strokeWidthMultiplier = builder.strokeWidthMultiplier;
        this.projectionOffsetXMultiplier = builder.projectionOffsetXMultiplier;
        this.projectionOffsetYMultiplier = builder.projectionOffsetYMultiplier;
        this.projectionTransparency = builder.projectionTransparency;
        this.colorMode = builder.colorMode;
        this.durationMs = builder.durationMs;
        this.fixedDurationMs = builder.fixedDurationMs;
        this.timeOffsetMs = builder.timeOffsetMs;
        this.maxOnScreen = builder.maxOnScreen;
        this.scrollAreaRatio = builder.scrollAreaRatio;
        this.scrollGapRatio = builder.scrollGapRatio;
        this.lineSpacing = builder.lineSpacing;
        this.maxScrollLines = builder.maxScrollLines;
        this.maxTopLines = builder.maxTopLines;
        this.maxBottomLines = builder.maxBottomLines;
        this.showScroll = builder.showScroll;
        this.showTop = builder.showTop;
        this.showBottom = builder.showBottom;
        this.showReverse = builder.showReverse;
        this.showPositioned = builder.showPositioned;
        this.showSubtitle = builder.showSubtitle;
        this.showSpecial = builder.showSpecial;
    }

    public static final class Builder {
        private float textScale = 1.0f;
        private float transparency = 0.0f;
        private boolean textBold;
        private int styleMode = STYLE_STROKE;
        private float shadowTransparency = 0.1f;
        private float strokeWidthMultiplier = 0.12f;
        private float projectionOffsetXMultiplier = 0.08f;
        private float projectionOffsetYMultiplier = 0.02f;
        private float projectionTransparency = 0.1f;
        private int colorMode = COLOR_MODE_DEFAULT;
        private long durationMs = 5000;
        private long fixedDurationMs = 4000;
        private long timeOffsetMs;
        private int maxOnScreen = 20;
        private float scrollAreaRatio = 1.0f;
        private float scrollGapRatio = 0.0f;
        private float lineSpacing = 1.0f;
        private int maxScrollLines = 8;
        private int maxTopLines = 5;
        private int maxBottomLines = 3;
        private boolean showScroll = true;
        private boolean showTop = true;
        private boolean showBottom = true;
        private boolean showReverse;
        private boolean showPositioned;
        private boolean showSubtitle;
        private boolean showSpecial;

        public Builder setTextScale(float textScale) { this.textScale = textScale; return this; }
        public Builder setTransparency(float transparency) { this.transparency = transparency; return this; }
        public Builder setTextBold(boolean textBold) { this.textBold = textBold; return this; }
        public Builder setStyleMode(int styleMode) { this.styleMode = styleMode; return this; }
        public Builder setShadowTransparency(float shadowTransparency) { this.shadowTransparency = shadowTransparency; return this; }
        public Builder setStrokeWidthMultiplier(float strokeWidthMultiplier) { this.strokeWidthMultiplier = strokeWidthMultiplier; return this; }
        public Builder setProjectionOffsetXMultiplier(float value) { this.projectionOffsetXMultiplier = value; return this; }
        public Builder setProjectionOffsetYMultiplier(float value) { this.projectionOffsetYMultiplier = value; return this; }
        public Builder setProjectionTransparency(float transparency) { this.projectionTransparency = transparency; return this; }
        public Builder setColorMode(int colorMode) { this.colorMode = colorMode; return this; }
        public Builder setDurationMs(long durationMs) { this.durationMs = durationMs; return this; }
        public Builder setFixedDurationMs(long fixedDurationMs) { this.fixedDurationMs = fixedDurationMs; return this; }
        public Builder setTimeOffsetMs(long timeOffsetMs) { this.timeOffsetMs = timeOffsetMs; return this; }
        public Builder setMaxOnScreen(int maxOnScreen) { this.maxOnScreen = maxOnScreen; return this; }
        public Builder setScrollAreaRatio(float scrollAreaRatio) { this.scrollAreaRatio = scrollAreaRatio; return this; }
        public Builder setScrollGapRatio(float scrollGapRatio) { this.scrollGapRatio = scrollGapRatio; return this; }
        public Builder setLineSpacing(float lineSpacing) { this.lineSpacing = lineSpacing; return this; }
        public Builder setMaxScrollLines(int maxScrollLines) { this.maxScrollLines = maxScrollLines; return this; }
        public Builder setMaxTopLines(int maxTopLines) { this.maxTopLines = maxTopLines; return this; }
        public Builder setMaxBottomLines(int maxBottomLines) { this.maxBottomLines = maxBottomLines; return this; }
        public Builder setShowScroll(boolean showScroll) { this.showScroll = showScroll; return this; }
        public Builder setShowTop(boolean showTop) { this.showTop = showTop; return this; }
        public Builder setShowBottom(boolean showBottom) { this.showBottom = showBottom; return this; }
        public Builder setShowReverse(boolean showReverse) { this.showReverse = showReverse; return this; }
        public Builder setShowPositioned(boolean showPositioned) { this.showPositioned = showPositioned; return this; }
        public Builder setShowSubtitle(boolean showSubtitle) { this.showSubtitle = showSubtitle; return this; }
        public Builder setShowSpecial(boolean showSpecial) { this.showSpecial = showSpecial; return this; }
        public DanmakuConfig build() { return new DanmakuConfig(this); }
    }
}
