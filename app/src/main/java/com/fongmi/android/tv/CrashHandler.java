package com.fongmi.android.tv;

import android.app.Activity;
import android.app.Application;
import android.content.Intent;
import android.os.Bundle;

import com.fongmi.android.tv.ui.activity.CrashActivity;
import com.fongmi.android.tv.ui.activity.HomeActivity;

import java.io.PrintWriter;
import java.io.StringWriter;

public class CrashHandler implements Application.ActivityLifecycleCallbacks {

    private static CrashHandler sInstance;
    private Thread.UncaughtExceptionHandler mDefaultHandler;

    public static void init() {
        if (sInstance == null) {
            sInstance = new CrashHandler();
        }
    }

    public static CrashHandler get() {
        return sInstance;
    }

    public void register(Application app) {
        mDefaultHandler = Thread.getDefaultUncaughtExceptionHandler();
        Thread.setDefaultUncaughtExceptionHandler(this::handleException);
        app.registerActivityLifecycleCallbacks(sInstance);
    }

    private void handleException(Thread t, Throwable e) {
        if (e == null) return;
        e.printStackTrace();
        Intent intent = new Intent(CrashActivity.ACTION_CRASH);
        intent.putExtra(CrashActivity.EXTRA_STACK_TRACE, getStackTrace(e));
        intent.putExtra(CrashActivity.EXTRA_EXCEPTION, e.getClass().getName());
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_CLEAR_TOP);
        try {
            CrashActivity.sActivity.startActivity(intent);
        } catch (Exception ex) {
            if (mDefaultHandler != null) mDefaultHandler.uncaughtException(t, e);
        }
    }

    private String getStackTrace(Throwable e) {
        StringWriter sw = new StringWriter();
        e.printStackTrace(new PrintWriter(sw));
        return sw.toString();
    }

    @Override public void onActivityCreated(Activity a, Bundle b) {}
    @Override public void onActivityStarted(Activity a) {}
    @Override public void onActivityResumed(Activity a) {}
    @Override public void onActivityPaused(Activity a) {}
    @Override public void onActivityStopped(Activity a) {}
    @Override public void onActivitySaveInstanceState(Activity a, Bundle b) {}
    @Override public void onActivityDestroyed(Activity a) {}
}
