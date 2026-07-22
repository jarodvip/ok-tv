package com.fongmi.android.tv.net;

import android.text.TextUtils;
import android.util.Log;

import com.fongmi.android.tv.api.config.RuleConfig;
import com.github.catvod.bean.Doh;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.util.List;

public final class RustDnsCallback {

    private static final String TAG = "RustDnsCallback";

    private RustDnsCallback() {
    }

    public static String buildConfigJson() {
        try {
            JSONObject json = new JSONObject();
            JSONArray doh = new JSONArray();
            JSONArray hosts = new JSONArray();

            for (Doh item : com.fongmi.android.tv.api.config.VodConfig.get().getDoh()) {
                if (TextUtils.isEmpty(item.getUrl())) continue;
                JSONObject obj = new JSONObject();
                obj.put("name", item.getName());
                obj.put("url", item.getUrl());
                JSONArray ips = new JSONArray();
                for (String ip : item.getIps()) ips.put(ip);
                obj.put("ips", ips);
                doh.put(obj);
            }

            for (String host : RuleConfig.get().getAds()) {
                if (!TextUtils.isEmpty(host)) hosts.put(host);
            }

            json.put("doh", doh);
            json.put("hosts", hosts);
            json.put("ttl_secs", 60);
            return json.toString();
        } catch (JSONException e) {
            Log.w(TAG, "buildConfigJson failed", e);
            return "{\"doh\":[],\"hosts\":[],\"ttl_secs\":60}";
        }
    }
}
