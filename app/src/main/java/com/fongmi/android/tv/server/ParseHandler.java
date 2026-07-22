package com.fongmi.android.tv.server;

import android.text.TextUtils;

import com.github.catvod.utils.Asset;

public final class ParseHandler {

    private ParseHandler() {
    }

    public static String handle(String query) {
        String jxs = QueryUtil.get(query, "jxs");
        String url = QueryUtil.get(query, "url");
        if (jxs == null) jxs = "";
        if (url == null) url = "";
        String template = Asset.read("parse.html");
        if (TextUtils.isEmpty(template)) template = "<html><body><pre>parse.html missing</pre></body></html>";
        return String.format(template, jxs.replace("%", "%%"), url.replace("%", "%%"));
    }
}
