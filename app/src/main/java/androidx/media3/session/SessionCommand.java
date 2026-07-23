package androidx.media3.session;

import android.os.Bundle;

import androidx.annotation.NonNull;

public final class SessionCommand implements androidx.media3.common.Bundleable {
    public static final int COMMAND_CODE_CUSTOM = 0;
    public static final int COMMAND_CODE_SESSION_SET_RATING = 1;
    public static final int COMMAND_CODE_LIBRARY_GET_LIBRARY_ROOT = 2;
    public static final int COMMAND_CODE_LIBRARY_SUBSCRIBE = 3;
    public static final int COMMAND_CODE_LIBRARY_UNSUBSCRIBE = 4;
    public static final int COMMAND_CODE_LIBRARY_GET_CHILDREN = 5;
    public static final int COMMAND_CODE_LIBRARY_GET_ITEM = 6;
    public static final int COMMAND_CODE_LIBRARY_SEARCH = 7;
    public static final int COMMAND_CODE_LIBRARY_GET_SEARCH_RESULT = 8;

    public final int commandCode;
    public final String customAction;
    public final Bundle customExtras;

    public SessionCommand(int commandCode) {
        this(commandCode, Bundle.EMPTY);
    }

    public SessionCommand(@NonNull String customAction, @Nullable Bundle customExtras) {
        this.commandCode = COMMAND_CODE_CUSTOM;
        this.customAction = customAction;
        this.customExtras = customExtras;
    }

    public boolean equals(Object obj) { return this == obj; }
    public int hashCode() { return System.identityHashCode(this); }
    public Bundle toBundle() { return Bundle.EMPTY; }
    public static SessionCommand fromBundle(Bundle bundle) { return new SessionCommand(COMMAND_CODE_CUSTOM); }
}
