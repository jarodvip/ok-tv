package com.fongmi.android.tv.ui;

import android.content.Context;
import android.util.AttributeSet;
import android.view.View;

import androidx.annotation.Nullable;

import androidx.media3.common.text.Cue;
import androidx.media3.common.text.CueGroup;
import androidx.media3.ui.CaptionStyleCompat;

import java.util.List;

public class CustomSubtitleView extends View {

    private float position;
    private float textSize;
    public static final float DEFAULT_TEXT_SIZE_FRACTION = 0.0625f;

    public CustomSubtitleView(Context context) { super(context); init(); }
    public CustomSubtitleView(Context context, @Nullable AttributeSet attrs) { super(context, attrs); init(); }
    public CustomSubtitleView(Context context, @Nullable AttributeSet attrs, int defStyleAttr) { super(context, attrs, defStyleAttr); init(); }

    private void init() {
        position = 0.0f;
        textSize = 0.0f;
    }

    public void setCues(@Nullable List<Cue> cues) {}
    public float getPosition() { return position; }
    public void addPosition(float delta) { position += delta; }
    public void subPosition(float delta) { position -= delta; }
    public float getTextSize() { return textSize; }
    public void addTextSize(float delta) { textSize += delta; }
    public void subTextSize(float delta) { textSize -= delta; }
    public void reset() { position = 0.0f; textSize = 0.0f; }

    public void setFractionalTextSize(float fraction) {}
    public void setFractionalTextSize(float fraction, boolean applyToTop) {}
    public void setApplyEmbeddedStyles(boolean apply) {}
    public void setApplyEmbeddedFontSizes(boolean apply) {}
    public void setBottomPaddingFraction(float fraction) {}
    public void setBottomPosition(float fraction) {}
    public void setStyle(@Nullable CaptionStyleCompat style) {}
}
