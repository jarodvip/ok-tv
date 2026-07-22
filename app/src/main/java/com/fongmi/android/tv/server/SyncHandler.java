package com.fongmi.android.tv.server;

import android.text.TextUtils;
import android.util.Log;

import com.fongmi.android.tv.App;
import com.fongmi.android.tv.Constant;
import com.fongmi.android.tv.api.config.VodConfig;
import com.fongmi.android.tv.bean.Config;
import com.fongmi.android.tv.bean.Device;
import com.fongmi.android.tv.bean.History;
import com.fongmi.android.tv.bean.Keep;
import com.fongmi.android.tv.event.RefreshEvent;
import com.fongmi.android.tv.event.ServerEvent;
import com.fongmi.android.tv.impl.Callback;
import com.fongmi.android.tv.utils.Notify;
import com.github.catvod.net.OkHttp;

import java.util.List;
import java.util.Map;
import java.util.Objects;

import okhttp3.FormBody;

public final class SyncHandler {

    private static final String TAG = "SyncHandler";

    private SyncHandler() {
    }

    public static String handle(Map<String, String> params) {
        String type = params == null ? null : params.get("type");
        if (TextUtils.isEmpty(type)) return "OK";
        if ("history".equals(type)) return handleHistory(params);
        if ("keep".equals(type)) return handleKeep(params);
        return "OK";
    }

    private static String handleHistory(Map<String, String> params) {
        String deviceJson = params.get("device");
        String mode = Objects.requireNonNullElse(params.get("mode"), "0");
        if (params.containsKey("device") && (Objects.equals(mode, "0") || Objects.equals(mode, "2"))) {
            Device device = Device.objectFrom(deviceJson);
            if (device != null && device.getIp() != null) sendHistory(device, params);
        }
        if (Objects.equals(mode, "0") || Objects.equals(mode, "1")) {
            syncHistory(params, Objects.equals(params.get("force"), "true"));
        }
        return "OK";
    }

    private static String handleKeep(Map<String, String> params) {
        String deviceJson = params.get("device");
        String mode = Objects.requireNonNullElse(params.get("mode"), "0");
        if (params.containsKey("device") && (Objects.equals(mode, "0") || Objects.equals(mode, "2"))) {
            Device device = Device.objectFrom(deviceJson);
            if (device != null && device.getIp() != null) sendKeep(device);
        }
        if (Objects.equals(mode, "0") || Objects.equals(mode, "1")) {
            syncKeep(params, Objects.equals(params.get("force"), "true"));
        }
        return "OK";
    }

    private static void sendHistory(Device device, Map<String, String> params) {
        try {
            Config config = Config.find(Config.objectFrom(params.get("config")));
            if (config.getUrl() == null) config = Config.vod();
            FormBody.Builder body = new FormBody.Builder();
            body.add("config", config.toString());
            body.add("targets", App.gson().toJson(History.get(config.getId())));
            post(device, "history", body);
        } catch (Throwable e) {
            App.post(() -> Notify.show(e.getMessage()));
        }
    }

    private static void sendKeep(Device device) {
        try {
            FormBody.Builder body = new FormBody.Builder();
            body.add("targets", App.gson().toJson(Keep.getVod()));
            body.add("configs", App.gson().toJson(Config.findUrls()));
            post(device, "keep", body);
        } catch (Throwable e) {
            App.post(() -> Notify.show(e.getMessage()));
        }
    }

    private static void post(Device device, String type, FormBody.Builder body) {
        try {
            OkHttp.newCall(OkHttp.client(Constant.TIMEOUT_SYNC), device.getIp().concat("/action?do=sync&mode=0&type=" + type), body.build()).execute();
        } catch (Throwable e) {
            App.post(() -> Notify.show(e.getMessage()));
        }
    }

    private static void syncHistory(Map<String, String> params, boolean force) {
        Config config = Config.find(Config.objectFrom(params.get("config")));
        List<History> targets = History.arrayFrom(params.get("targets"));
        if (config.getUrl() == null) return;
        if (config.getUrl().equals(VodConfig.getUrl())) {
            if (force) History.delete(config.getId());
            History.sync(targets);
            RefreshEvent.history();
        } else {
            VodConfig.load(config, getHistoryCallback(targets, force, config.getId()));
        }
    }

    private static Callback getHistoryCallback(List<History> targets, boolean force, int cid) {
        return new Callback() {
            @Override
            public void success() {
                if (force) History.delete(cid);
                History.sync(targets);
                RefreshEvent.history();
            }

            @Override
            public void error(String msg) {
                Notify.show(msg);
            }
        };
    }

    private static void syncKeep(Map<String, String> params, boolean force) {
        List<Keep> targets = Keep.arrayFrom(params.get("targets"));
        List<Config> configs = Config.arrayFrom(params.get("configs"));
        if (TextUtils.isEmpty(VodConfig.getUrl()) && !configs.isEmpty()) {
            VodConfig.load(Config.find(configs.get(0)), getKeepCallback(configs, targets, force));
        } else {
            if (force) Keep.deleteAll();
            Keep.sync(configs, targets);
            RefreshEvent.keep();
        }
    }

    private static Callback getKeepCallback(List<Config> configs, List<Keep> targets, boolean force) {
        return new Callback() {
            @Override
            public void success() {
                if (force) Keep.deleteAll();
                Keep.sync(configs, targets);
                RefreshEvent.keep();
            }

            @Override
            public void error(String msg) {
                Notify.show(msg);
            }
        };
    }
}
