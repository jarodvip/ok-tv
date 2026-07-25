package com.github.catvod.net.interceptor;

import android.text.TextUtils;
import android.util.Log;

import com.github.catvod.net.RustNet;

import okhttp3.HttpUrl;
import okhttp3.Interceptor;
import okhttp3.Request;
import okhttp3.Response;

import java.io.IOException;
import java.util.Iterator;
import java.util.concurrent.ConcurrentHashMap;

public class RequestInterceptor implements Interceptor {

    private final ConcurrentHashMap<String, String> authMap;

    public RequestInterceptor() {
        authMap = new ConcurrentHashMap<>();
    }

    public void clear() {
        authMap.clear();
    }

    @Override
    public Response intercept(Chain chain) throws IOException {
        Request request = chain.request();
        Request.Builder builder = request.newBuilder();
        HttpUrl url = request.url();
        checkAuth(url, builder);
        injectHeaders(url, builder);
        return chain.proceed(builder.build());
    }

    private void checkAuth(HttpUrl url, Request.Builder builder) {
        String auth = url.queryParameter("auth");
        if (auth != null) {
            authMap.put(url.host(), auth);
        } else if (authMap.containsKey(url.host())) {
            builder.url(url.newBuilder().addQueryParameter("auth", authMap.get(url.host())).build());
        }
    }

    private void injectHeaders(HttpUrl url, Request.Builder builder) {
        try {
            String host = url.host();
            if (TextUtils.isEmpty(host)) return;
            String headersJson = RustNet.injectHeaders(host, "{}");
            if (TextUtils.isEmpty(headersJson)) return;
            org.json.JSONObject obj = new org.json.JSONObject(headersJson);
            for (Iterator<String> keys = obj.keys(); keys.hasNext(); ) {
                String key = keys.next();
                String value = obj.optString(key);
                if (!TextUtils.isEmpty(key) && !TextUtils.isEmpty(value)) {
                    builder.header(key, value);
                }
            }
        } catch (Exception e) {
            Log.w("RequestInterceptor", "inject rust headers failed", e);
        }
    }
}
