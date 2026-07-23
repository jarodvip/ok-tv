package androidx.media3.session;

public final class SessionError {
    public static final SessionError ERROR_BAD_VALUE = new SessionError(1);
    public final int errorCode;

    private SessionError(int errorCode) {
        this.errorCode = errorCode;
    }
}
