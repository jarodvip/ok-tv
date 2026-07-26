package com.fongmi.android.tv.ui.activity;

import android.app.Activity;
import android.app.Application;
import android.content.Intent;
import android.os.Bundle;

import androidx.appcompat.app.AlertDialog;

import com.fongmi.android.tv.R;
import com.fongmi.android.tv.CrashHandler;
import com.fongmi.android.tv.databinding.ActivityCrashBinding;
import com.fongmi.android.tv.ui.base.BaseActivity;
import com.github.catvod.utils.Prefers;

import java.io.PrintWriter;
import java.io.StringWriter;

public class CrashActivity extends BaseActivity {

    public static final String ACTION_CRASH = "com.fongmi.android.tv.CRASH";
    public static final String EXTRA_STACK_TRACE = "stack_trace";
    public static final String EXTRA_EXCEPTION = "exception";
    public static Activity sActivity;

    private ActivityCrashBinding mBinding;

    @Override
    protected boolean customWall() {
        return false;
    }

    @Override
    protected ActivityCrashBinding getBinding() {
        return mBinding = ActivityCrashBinding.inflate(getLayoutInflater());
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        sActivity = this;
        initView();
        setCrash();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        if (sActivity == this) {
            sActivity = null;
        }
    }

    private void initView() {
        mBinding.details.setOnClickListener(v -> showError());
        mBinding.restart.setOnClickListener(v -> {
            android.os.Process.killProcess(android.os.Process.myPid());
            System.exit(10);
        });
    }

    private void setCrash() {
        String log = getIntent().getStringExtra(EXTRA_STACK_TRACE);
        if (log == null) return;
        String[] lines = log.split("\n");
        for (int i = lines.length - 1; i >= 0; i--) {
            if (lines[i].isEmpty()) continue;
            if (lines[i].contains(HomeActivity.class.getSimpleName())) {
                Prefers.put("crash", true);
                break;
            }
        }
    }

    private void showError() {
        String stackTrace = getIntent().getStringExtra(EXTRA_STACK_TRACE);
        String exception = getIntent().getStringExtra(EXTRA_EXCEPTION);
        StringBuilder msg = new StringBuilder();
        msg.append(exception != null ? exception : "Unknown").append("\n\n");
        if (stackTrace != null) msg.append(stackTrace);
        new AlertDialog.Builder(this)
                .setTitle(R.string.crash_details_title)
                .setMessage(msg)
                .setPositiveButton(R.string.crash_details_close, null)
                .show();
    }
}
