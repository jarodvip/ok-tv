package com.fongmi.android.tv.ui;

import android.content.Context;
import android.util.AttributeSet;
import android.view.View;
import android.widget.FrameLayout;

import androidx.annotation.Nullable;

import androidx.media3.common.Player;
import androidx.media3.common.text.Cue;
import androidx.media3.common.text.CueGroup;
import androidx.media3.ui.CaptionStyleCompat;
import androidx.media3.ui.PlayerView;
import androidx.media3.ui.SubtitleView;

import java.util.List;

public class CustomPlayerView extends FrameLayout {

    private final PlayerView playerView;
    private final CustomSubtitleView subtitleView;

    public CustomPlayerView(Context context) {
        this(context, null);
    }

    public CustomPlayerView(Context context, @Nullable AttributeSet attrs) {
        this(context, attrs, 0);
    }

    public CustomPlayerView(Context context, @Nullable AttributeSet attrs, int defStyleAttr) {
        super(context, attrs, defStyleAttr);
        playerView = new PlayerView(context, attrs, defStyleAttr);
        subtitleView = new CustomSubtitleView(context, attrs);
        addView(playerView, new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT));
    }

    public Player getPlayer() { return playerView.getPlayer(); }
    public void setPlayer(@Nullable Player player) { playerView.setPlayer(player); }
    public void setResizeMode(int resizeMode) { playerView.setResizeMode(resizeMode); }
    public int getResizeMode() { return playerView.getResizeMode(); }
    public void setDefaultArtwork(android.graphics.drawable.Drawable artwork) { playerView.setDefaultArtwork(artwork); }
    public android.graphics.drawable.Drawable getDefaultArtwork() { return playerView.getDefaultArtwork(); }
    public void setVisibility(int visibility) { super.setVisibility(visibility); playerView.setVisibility(visibility); }
    public void setControllerShowTimeoutMs(int timeout) { playerView.setControllerShowTimeoutMs(timeout); }
    public int getControllerShowTimeoutMs() { return playerView.getControllerShowTimeoutMs(); }
    public void showController() { playerView.showController(); }
    public void hideController() { playerView.hideController(); }

    public void mute() {}
    public void setDanmakuConfig(Object config) {}
    public void setSubtitleStyle(Object config) {}
    public CustomSubtitleView getSubtitleView() { return subtitleView; }
    public void setSubtitleView(CustomSubtitleView view) {}
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
}
