package androidx.media3.session;

import android.content.ComponentName;
import android.content.Context;
import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

public final class SessionToken implements androidx.media3.common.Bundleable {
    public static final int TYPE_SESSION = 0;
    public static final int TYPE_SESSION_SERVICE = 1;
    public static final int TYPE_LIBRARY_SERVICE = 2;

    public SessionToken(@NonNull Context context, @NonNull ComponentName serviceComponent) {}

    @Override
    public boolean equals(Object obj) { return this == obj; }
    @Override
    public int hashCode() { return System.identityHashCode(this); }
    public Bundle toBundle() { return Bundle.EMPTY; }
    public static SessionToken fromBundle(Bundle bundle) { return new SessionToken(null, null); }
}
