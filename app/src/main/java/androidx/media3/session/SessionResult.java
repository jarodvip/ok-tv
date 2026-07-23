package androidx.media3.session;

import android.os.Bundle;

public final class SessionResult implements androidx.media3.common.Bundleable {
    public static final int RESULT_SUCCESS = 0;
    public static final int RESULT_ERROR_UNKNOWN = 1;

    public final int resultCode;
    public final Bundle extras;
    public final long completionTimeMs;

    public SessionResult(int resultCode) { this(resultCode, Bundle.EMPTY); }
    public SessionResult(int resultCode, Bundle extras) {
        this.resultCode = resultCode;
        this.extras = extras;
        this.completionTimeMs = System.currentTimeMillis();
    }

    public boolean equals(Object obj) { return this == obj; }
    public int hashCode() { return System.identityHashCode(this); }
    public Bundle toBundle() { return extras; }
    public static SessionResult fromBundle(Bundle bundle) { return new SessionResult(RESULT_SUCCESS, bundle); }
}
