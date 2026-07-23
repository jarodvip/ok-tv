package androidx.media3.session;

import android.net.Uri;
import android.os.Bundle;

import androidx.annotation.NonNull;

public final class CommandButton {

    public static final int ICON_STOP = 1;
    public static final int ICON_REPEAT_OFF = 2;
    public static final int ICON_REPEAT_ONE = 3;

    public final SessionCommand sessionCommand;
    public final int playerCommand;
    public final int iconResId;
    public final Uri iconUri;
    public final CharSequence displayName;
    public final Bundle extras;
    public final boolean isEnabled;

    private CommandButton(Builder builder) {
        this.sessionCommand = builder.sessionCommand;
        this.playerCommand = builder.playerCommand;
        this.iconResId = builder.iconResId;
        this.iconUri = builder.iconUri;
        this.displayName = builder.displayName;
        this.extras = builder.extras;
        this.isEnabled = builder.isEnabled;
    }

    public static final class Builder {
        private SessionCommand sessionCommand;
        private int playerCommand;
        private int iconResId;
        private Uri iconUri;
        private CharSequence displayName;
        private Bundle extras;
        private boolean isEnabled = true;

        public Builder() {}

        public Builder setSessionCommand(SessionCommand sessionCommand) {
            this.sessionCommand = sessionCommand;
            return this;
        }

        public Builder setPlayerCommand(int playerCommand) {
            this.playerCommand = playerCommand;
            return this;
        }

        public Builder setIconResId(int iconResId) {
            this.iconResId = iconResId;
            return this;
        }

        public Builder setIconUri(Uri iconUri) {
            this.iconUri = iconUri;
            return this;
        }

        public Builder setDisplayName(CharSequence displayName) {
            this.displayName = displayName;
            return this;
        }

        public Builder setEnabled(boolean enabled) {
            this.isEnabled = enabled;
            return this;
        }

        public Builder setExtras(Bundle extras) {
            this.extras = extras;
            return this;
        }

        public CommandButton build() {
            return new CommandButton(this);
        }
    }
}
