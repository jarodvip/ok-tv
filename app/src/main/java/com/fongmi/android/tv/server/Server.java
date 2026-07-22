package com.fongmi.android.tv.server;

import android.util.Log;

import com.fongmi.android.tv.service.PlaybackService;
import com.fongmi.android.tv.utils.Task;
import com.github.catvod.Proxy;
import com.github.catvod.utils.Util;

public class Server {

    private static final String TAG = "LocalServer";

    private volatile PlaybackService service;
    private volatile Nano nano;

    private static class Loader {
        static volatile Server INSTANCE = new Server();
    }

    public static Server get() {
        return Loader.INSTANCE;
    }

    public PlaybackService getService() {
        return service;
    }

    public void setService(PlaybackService service) {
        this.service = service;
    }

    public String getAddress() {
        return getAddress(false);
    }

    public String getAddress(int tab) {
        return getAddress(false) + "?tab=" + tab;
    }

    public String getAddress(String path) {
        return getAddress(true) + path;
    }

    public String getAddress(boolean local) {
        return "http://" + (local ? "127.0.0.1" : Util.getIp()) + ":" + Proxy.getPort();
    }

    public synchronized void start() {
        if (nano != null) return;

        if (tryStartRust()) {
            Log.i(TAG, "started with rust server on " + Proxy.getPort());
            return;
        }

        startNano();
    }

    private boolean tryStartRust() {
        if (!RustServer.isStarted()) {
            boolean ok = RustServer.start(9978, 9998);
            if (!ok) return false;
        }

        int port = RustServer.getPort();
        if (port <= 0) return false;

        Proxy.set(port);
        return true;
    }

    private void startNano() {
        for (int i = 9978; i < 9999; i++) {
            try {
                nano = new Nano(i);
                nano.start(500);
                Proxy.set(i);
                Log.i(TAG, "started with nano server on " + i);
                return;
            } catch (Throwable e) {
                nano = null;
            }
        }
    }

    public void stop() {
        Task.execute(() -> {
            if (nano != null) nano.stop();
            RustServer.stop();
            service = null;
            nano = null;
        });
    }
}
