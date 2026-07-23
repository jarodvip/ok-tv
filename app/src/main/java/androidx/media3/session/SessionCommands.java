package androidx.media3.session;

import android.os.Bundle;

import androidx.annotation.NonNull;

import com.google.common.collect.ImmutableSet;

import java.util.Collection;

public final class SessionCommands implements androidx.media3.common.Bundleable {
    public static final SessionCommands EMPTY = new SessionCommands(ImmutableSet.of());
    public final ImmutableSet<SessionCommand> commands;

    private SessionCommands(ImmutableSet<SessionCommand> commands) { this.commands = commands; }

    public boolean contains(SessionCommand command) { return commands.contains(command); }
    public boolean contains(int commandCode) { return false; }
    public SessionCommands.Builder buildUpon() { return new Builder(this); }
    public boolean equals(Object obj) { return this == obj; }
    public int hashCode() { return System.identityHashCode(this); }
    public Bundle toBundle() { return Bundle.EMPTY; }
    public static SessionCommands fromBundle(Bundle bundle) { return EMPTY; }

    public static final class Builder {
        private ImmutableSet<SessionCommand> commands;

        public Builder() {}
        public Builder(SessionCommands commands) { this.commands = commands.commands; }
        public Builder add(@NonNull SessionCommand command) { this.commands = ImmutableSet.builder().addAll(commands).add(command).build(); return this; }
        public Builder addAll(@NonNull Collection<SessionCommand> commands) { this.commands = ImmutableSet.builder().addAll(this.commands).addAll(commands).build(); return this; }
        public Builder remove(@NonNull SessionCommand command) { return this; }
        public SessionCommands build() { return new SessionCommands(commands); }
    }
}
