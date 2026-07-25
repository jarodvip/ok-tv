package com.github.catvod.net;

import android.text.TextUtils;
import android.util.Log;

import com.github.catvod.net.RustNet;
import com.github.catvod.utils.Util;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.IOException;
import java.net.Authenticator;
import java.net.Proxy;
import java.net.ProxySelector;
import java.net.SocketAddress;
import java.net.URI;
import java.net.InetSocketAddress;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

public class OkProxySelector extends ProxySelector {

    private static final String TAG = "OkProxySelector";

    private final List<com.github.catvod.bean.Proxy> proxy;
    private final ProxySelector system;
    private boolean authSet;

    public OkProxySelector() {
        proxy = new CopyOnWriteArrayList<>();
        system = ProxySelector.getDefault();
        Authenticator.setDefault(new ProxyAuthenticator(this));
    }

    public synchronized void addAll(List<com.github.catvod.bean.Proxy> items) {
        if (items.isEmpty()) return;
        items.forEach(com.github.catvod.bean.Proxy::init);
        proxy.addAll(items);
        proxy.sort(null);
    }

    public synchronized void clear() {
        Authenticator.setDefault(null);
        proxy.clear();
    }

    public List<com.github.catvod.bean.Proxy> getProxy() {
        return proxy;
    }

    private List<java.net.Proxy> fallback(URI uri) {
        return system != null ? system.select(uri) : List.of(java.net.Proxy.NO_PROXY);
    }

    @Override
    public List<java.net.Proxy> select(URI uri) {
        if (uri == null || uri.getHost() == null || "127.0.0.1".equals(uri.getHost())) return fallback(uri);
        String host = uri.getHost();

        List<java.net.Proxy> rustProxy = resolveRustProxy(host);
        if (!rustProxy.isEmpty()) {
            return rustProxy;
        }

        if (proxy.isEmpty()) return fallback(uri);
        for (com.github.catvod.bean.Proxy item : proxy) {
            for (String pattern : item.getHosts()) {
                if (Util.containOrMatch(host, pattern)) {
                    return !item.getProxies().isEmpty() ? item.getProxies() : fallback(uri);
                }
            }
        }
        return fallback(uri);
    }

    private List<java.net.Proxy> resolveRustProxy(String host) {
        try {
            String json = RustNet.resolveProxy(host);
            if (TextUtils.isEmpty(json)) return List.of();
            JSONObject obj = new JSONObject(json);
            String proxyType = obj.optString("proxy_type", "");
            String hostname = obj.optString("hostname", "");
            int port = obj.optInt("port", 0);
            if (proxyType.isEmpty() || hostname.isEmpty() || port <= 0) return List.of();
            java.net.Proxy.Type type = "socks".equalsIgnoreCase(proxyType) ? java.net.Proxy.Type.SOCKS : java.net.Proxy.Type.HTTP;
            InetSocketAddress address = InetSocketAddress.createUnresolved(hostname, port);
            return List.of(new java.net.Proxy(type, address));
        } catch (Exception e) {
            Log.w(TAG, "resolve rust proxy failed", e);
            return List.of();
        }
    }

    @Override
    public void connectFailed(URI uri, SocketAddress socketAddress, IOException e) {
        if (system != null) system.connectFailed(uri, socketAddress, e);
    }
}
