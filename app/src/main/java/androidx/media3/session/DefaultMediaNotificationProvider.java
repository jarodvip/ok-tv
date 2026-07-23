package androidx.media3.session;

import android.content.Context;
import android.os.Bundle;

import com.google.common.collect.ImmutableList;

public class DefaultMediaNotificationProvider implements MediaNotification.Provider {

    public static final int DEFAULT_NOTIFICATION_ID = 0;
    public static final String DEFAULT_CHANNEL_ID = "media_playback";
    public static final String GROUP_KEY = "media_playback_group";

    public DefaultMediaNotificationProvider(Context context) {}
    public DefaultMediaNotificationProvider(Context context, NotificationIdProvider notificationIdProvider, String groupKey, int notificationId) {}

    @Override
    public MediaNotification createNotification(MediaSession session, ImmutableList<CommandButton> customLayout, MediaNotification.ActionFactory actionFactory, MediaNotification.Provider.Callback callback) {
        return null;
    }

    @Override
    public boolean handleCustomCommand(MediaSession session, String action, Bundle args) {
        return false;
    }

    public void setSmallIcon(int icon) {}

    public interface NotificationIdProvider {
        int getNotificationId(MediaSession session, ImmutableList<CommandButton> customLayout);
    }
}
