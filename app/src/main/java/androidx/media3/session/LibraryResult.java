package androidx.media3.session;

import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import com.google.common.collect.ImmutableList;
import com.google.common.util.concurrent.ListenableFuture;

import java.util.List;

public final class LibraryResult<V> implements androidx.media3.common.Bundleable {
    public static final int RESULT_SUCCESS = 0;
    public static final int RESULT_ERROR_UNKNOWN = 1;

    public final int resultCode;
    public final long completionTimeMs;
    public final V value;
    public final MediaLibraryService.LibraryParams params;

    public static <V> LibraryResult<V> ofError(int resultCode) { return new LibraryResult<>(resultCode, null, null); }
    public static <V> LibraryResult<V> ofError(int resultCode, MediaLibraryService.LibraryParams params) { return new LibraryResult<>(resultCode, null, params); }
    public static LibraryResult<Void> ofVoid() { return new LibraryResult<>(RESULT_SUCCESS, null, null); }
    public static LibraryResult<Void> ofVoid(MediaLibraryService.LibraryParams params) { return new LibraryResult<>(RESULT_SUCCESS, null, params); }
    public static LibraryResult<androidx.media3.common.MediaItem> ofItem(androidx.media3.common.MediaItem value, MediaLibraryService.LibraryParams params) { return new LibraryResult<>(RESULT_SUCCESS, value, params); }
    public static LibraryResult<ImmutableList<androidx.media3.common.MediaItem>> ofItemList(List<androidx.media3.common.MediaItem> value, MediaLibraryService.LibraryParams params) { return new LibraryResult<>(RESULT_SUCCESS, value, params); }
    public Bundle toBundle() { return Bundle.EMPTY; }
    public static LibraryResult<Void> fromVoidBundle(Bundle bundle) { return ofVoid(); }
    public static LibraryResult<androidx.media3.common.MediaItem> fromItemBundle(Bundle bundle) { return ofItem(null, null); }
    public static LibraryResult<ImmutableList<androidx.media3.common.MediaItem>> fromItemListBundle(Bundle bundle) { return ofItemList(ImmutableList.of(), null); }
    public static LibraryResult<?> fromUnknownBundle(Bundle bundle) { return ofError(RESULT_ERROR_UNKNOWN); }

    private LibraryResult(int resultCode, V value, MediaLibraryService.LibraryParams params) {
        this.resultCode = resultCode;
        this.completionTimeMs = System.currentTimeMillis();
        this.value = value;
        this.params = params;
    }
}
