package com.fongmi.android.tv.net;

import android.net.Uri;
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

            List<com.github.catvod.bean.Proxy> proxyList = getProxyList();
            Map<String, String> proxyEndpoint = parseProxyEndpoints(proxyList);

            for (com.fongmi.android.tv.bean.Rule rule : RuleConfig.get().getRules()) {
                JSONObject proxy = new JSONObject();
                proxy.put("host", TextUtils.join(",", rule.getHosts()));
                proxy.put("proxy_type", rule.getName());
                String endpoint = proxyEndpoint.get(rule.getName());
                if (endpoint != null) {
                    Uri uri = Uri.parse(endpoint);
                    proxy.put("hostname", uri.getHost());
                    proxy.put("port", uri.getPort());
                } else {
                    proxy.put("hostname", "");
                    proxy.put("port", 0);
                }
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

    private static List<com.github.catvod.bean.Proxy> getProxyList() {
        try {
            com.github.catvod.net.OkProxySelector selector = com.github.catvod.net.OkHttp.selector();
            if (selector != null) return selector.getProxy();
        } catch (Exception e) {
            // ignore
        }
        return List.of();
    }

    private static java.util.Map<String, String> parseProxyEndpoints(List<com.github.catvod.bean.Proxy> proxyList) {
        java.util.Map<String, String> result = new java.util.HashMap<>();
        for (com.github.catvod.bean.Proxy p : proxyList) {
            List<String> urls = p.getUrls();
            if (urls != null && !urls.isEmpty()) {
                result.put(p.getName(), urls.get(0));
            }
        }
        return result;
    }

    public static String getHosts() {
        return TextUtils.join("\n", RuleConfig.get().getAds());
    }

    public static String getRules() {
        return "";
    }
}
