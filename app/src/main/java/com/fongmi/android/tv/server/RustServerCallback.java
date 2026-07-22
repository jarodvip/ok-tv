package com.fongmi.android.tv.server;

public final class RustServerCallback {

    private RustServerCallback() {
    }

    public static String onHandle(String path, String query, byte[] body) {
        if ("/device".equals(path)) {
            return DeviceHandler.handle();
        }

        if ("/tvbus".equals(path)) {
            return TvbusHandler.handle(query);
        }

        if ("/action".equals(path)) {
            return ActionHandler.handle(query);
        }

        if ("/media".equals(path)) {
            return MediaHandler.handle();
        }

        if ("/cache".equals(path)) {
            return CacheHandler.handle(query);
        }

        if ("/parse".equals(path)) {
            return ParseHandler.handle(query);
        }

        if ("/proxy".equals(path)) {
            return ProxyHandler.handle(query, body);
        }

        if ("/newFolder".equals(path)) {
            return NewFolderHandler.handle(query);
        }

        if ("/delFile".equals(path)) {
            return DelFileHandler.handle(QueryUtil.get(query, "path"));
        }

        if ("/delFolder".equals(path)) {
            return DelFolderHandler.handle(QueryUtil.get(query, "path"));
        }

        if ("/file".equals(path)) {
            return FileHandler.handle(QueryUtil.get(query, "path"), QueryUtil.get(query, "range"));
        }

        if ("/upload".equals(path)) {
            return UploadHandler.handle(query, body);
        }

        return "OK";
    }
}
