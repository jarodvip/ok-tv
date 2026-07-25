package com.fongmi.android.tv.ui;

import android.content.Context;
import android.util.AttributeSet;
import android.view.View;

import androidx.annotation.NonNull;

import androidx.media3.common.Player;
import androidx.media3.ui.TimeBar;

public class PlayerSeekView extends View {
    private Player player;

    public PlayerSeekView(Context context) { super(context); }
    public PlayerSeekView(Context context, AttributeSet attrs) { super(context, attrs); }
    public PlayerSeekView(Context context, AttributeSet attrs, int defStyleAttr) { super(context, attrs, defStyleAttr); }

    public void setPlayer(@NonNull Player player) { this.player = player; }
    public Player getPlayer() { return player; }
    public TimeBar getTimeBar() { return null; }
    public void setKeyTimeIncrement(long timeMs) {}
}
