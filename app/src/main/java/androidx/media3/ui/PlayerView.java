package androidx.media3.ui;

import android.content.Context;
import android.util.AttributeSet;
import android.view.View;
import android.widget.FrameLayout;

import androidx.annotation.Nullable;

import androidx.media3.common.Player;
import androidx.media3.common.text.Cue;
import androidx.media3.ui.CaptionStyleCompat;

public class PlayerView extends FrameLayout {

    private Player player;

    public PlayerView(Context context) { super(context); }
    public PlayerView(Context context, @Nullable AttributeSet attrs) { super(context, attrs); }
    public PlayerView(Context context, @Nullable AttributeSet attrs, int defStyleAttr) { super(context, attrs, defStyleAttr); }

    public Player getPlayer() { return player; }
    public void setPlayer(@Nullable Player player) { this.player = player; }

    public void mute() {}
    public void setDanmakuConfig(Object config) {}
    public void setSubtitleStyle(Object config) {}
    public SubtitleView getSubtitleView() { return new SubtitleView(getContext()); }
    public void setSubtitleView(SubtitleView view) {}
    public void setApplyEmbeddedStyles(boolean apply) {}
    public void setApplyEmbeddedFontSizes(boolean apply) {}
    public void setBottomPaddingFraction(float fraction) {}
    public void setFractionalTextSize(float fraction) {}
    public void setFractionalTextSize(float fraction, boolean applyToTop) {}
    public void setStyle(CaptionStyleCompat style) {}
    public void setDanmakuSource(Object uri) {}
    public void setDanmakuEnabled(boolean enabled) {}
    public void setDanmakuOkHttpClient(Object client) {}
    public boolean isDebugViewVisible() { return false; }
    public void toggleDebugView() {}
    public void hideDebugView() {}
    public void setRender(int render) {}
    public void sendDanmaku(String text) {}

    @Override
    public void setVisibility(int visibility) {
        super.setVisibility(visibility);
    }
}
