package com.fongmi.android.tv.net;

import android.text.TextUtils;
import android.util.Log;

import com.fongmi.android.tv.api.config.RuleConfig;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.util.List;
import java.util.Map;

public final class RustNetCallback {

    private static final String TAG = "RustNetCallback";

    private RustNetCallback() {
    }

    public static String buildRulesJson() {
        try {
            JSONObject json = new JSONObject();
            JSONArray proxies = new JSONArray();
            JSONArray headers = new JSONArray();
            JSONArray ads = new JSONArray();

            for (com.fongmi.android.tv.bean.Rule rule : RuleConfig.get().getRules()) {
                JSONObject proxy = new JSONObject();
                proxy.put("host", TextUtils.join(",", rule.getHosts()));
                proxy.put("proxy_type", rule.getName());
                proxy.put("hostname", "");
                proxy.put("port", 0);
                proxies.put(proxy);

                JSONObject header = new JSONObject();
                header.put("host", TextUtils.join(",", rule.getHosts()));
                JSONObject headerMap = new JSONObject();
                header.put("headers", headerMap);
                headers.put(header);
            }

            for (String ad : RuleConfig.get().getAds()) {
                ads.put(ad);
            }

            json.put("proxies", proxies);
            json.put("headers", headers);
            json.put("ads", ads);
            return json.toString();
        } catch (JSONException e) {
            Log.w(TAG, "buildRulesJson failed", e);
            return "{\"proxies\":[],\"headers\":[],\"ads\":[]}";
        }
    }

    public static String getHosts() {
        return TextUtils.join("\n", RuleConfig.get().getAds());
    }

    public static String getRules() {
        return "";
    }
}
