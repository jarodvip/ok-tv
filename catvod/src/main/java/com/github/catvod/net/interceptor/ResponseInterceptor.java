package com.github.catvod.net.interceptor;

import android.text.TextUtils;
import android.util.Log;

import com.github.catvod.net.RustNet;

import okhttp3.Interceptor;
import okhttp3.Request;
import okhttp3.Response;
import okhttp3.ResponseBody;
import okhttp3.MediaType;
import okio.BufferedSource;
import okio.Okio;

import org.json.JSONException;
import org.json.JSONObject;

import com.github.catvod.bean.Header;

import java.io.IOException;
import java.io.InputStream;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.zip.Inflater;
import java.util.zip.InflaterInputStream;

public class ResponseInterceptor implements Interceptor {

    private final ConcurrentHashMap<String, String> redirectMap;

    public ResponseInterceptor() {
        redirectMap = new ConcurrentHashMap<>();
    }

    public void clear() {
        redirectMap.clear();
    }

    public void addAll(List<Header> items) {}

    @Override
    public Response intercept(Chain chain) throws IOException {
        Request request = chain.request();
        Response response = chain.proceed(request);

        String host = request.url().host();
        if (!TextUtils.isEmpty(host)) {
            response = handleRustRules(host, request, response);
        }

        String encoding = response.header("Content-Encoding");
        if ("deflate".equalsIgnoreCase(encoding)) return deflate(response);
        if (response.code() == 406 && redirectMap.containsKey(request.url().toString())) return redirect(request, response);
        if (response.code() == 302 && response.header("Location") != null) {
            redirectMap.put(response.header("Location"), request.url().toString());
        }
        return response;
    }

    private Response handleRustRules(String host, Request request, Response response) {
        try {
            if (RustNet.shouldBlock(request.url().toString())) {
                return new Response.Builder()
                        .request(response.request())
                        .protocol(response.protocol())
                        .code(403)
                        .message("Blocked")
                        .body(ResponseBody.create(response.body().contentType(), new byte[0]))
                        .build();
            }
        } catch (Exception e) {
            Log.w("ResponseInterceptor", "rust block check failed", e);
        }
        return response;
    }

    private Response redirect(Request request, Response response) {
        return new Response.Builder().request(request).protocol(response.protocol()).code(302).message("Found").header("Location", redirectMap.get(request.url().toString())).build();
    }

    private Response deflate(Response response) {
        InputStream is = new InflaterInputStream(response.body().byteStream(), new Inflater(true));
        return response.newBuilder().headers(response.headers()).body(getBody(response, is)).build();
    }

    private ResponseBody getBody(Response response, InputStream is) {
        return new ResponseBody() {
            @Override
            public MediaType contentType() {
                return response.body().contentType();
            }

            @Override
            public long contentLength() {
                return -1;
            }

            @Override
            public BufferedSource source() {
                return Okio.buffer(Okio.source(is));
            }
        };
    }
}
