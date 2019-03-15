use crate::trovo::Capability;

#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

/// Configuration used to complete the 'registration' with the irc server
pub struct UserConfig {
    /// OAuth token from trovo, it must have the
    /// [scopes](https://dev.trovo.tv/docs/authentication/#scopes):
    /// `chat:read`, `chat:edit`
    pub token: String,
    /// Username to use on trovo. (must be associated with the oauth token)
    pub nick: String,
    /// Which capabilites to enable
    pub caps: Vec<Capability>,
}

impl UserConfig {
    /// Create a [`UserConfigBuilder`](./userconfig/struct.UserConfigBuilder.html), defaults with all of the [`Capabilities`](./enum.Capability.html) enabled
    pub fn builder() -> UserConfigBuilder {
        UserConfigBuilder::new()
    }
}

/// A _builder_ type to create a [`UserConfig`](./struct.UserConfig.html) without dumb errors (like swapping nick/token)
pub struct UserConfigBuilder {
    nick: Option<String>,
    token: Option<String>,
    caps: HashSet<Capability>,
}

impl Default for UserConfigBuilder {
    fn default() -> Self {
        Self {
            nick: None,
            token: None,
            caps: [
                Capability::Membership,
                Capability::Commands,
                Capability::Tags,
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

impl UserConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use this nickname in the configuration
    pub fn nick<S: ToString>(mut self, nick: S) -> Self {
        let _ = self.nick.replace(nick.to_string());
        self
    }

    /// Use this oauth token in the configuration
    // check for the leading 'oauth:'
    // and probably the length (its probably 64 bytes)
    pub fn token<S: ToString>(mut self, token: S) -> Self {
        let _ = self.token.replace(token.to_string());
        self
    }

    /// Enable or disable the membership capability
    ///
    /// Enabled by default
    pub fn membership(mut self) -> Self {
        self.toggle_cap(Capability::Membership);
        self
    }

    /// Enable or disable the commands capability
    ///
    /// Enabled by default

    pub fn commands(mut self) -> Self {
        self.toggle_cap(Capability::Commands);
        self
    }

    /// Enable or disable the tags capability
    ///
    /// Enabled by default
    pub fn tags(mut self) -> Self {
        self.toggle_cap(Capability::Tags);
        self
    }

    /// Build the `UserConfig`
    ///
    /// Returns None if nick or token are invalid
    pub fn build(self) -> Option<UserConfig> {
        Some(UserConfig {
            nick: self.nick?,
            token: self.token?,
            caps: self.caps.into_iter().collect(),
        })
    }

    fn toggle_cap(&mut self, cap: Capability) {
        if self.caps.contains(&cap) {
            let _ = self.caps.remove(&cap);
        } else {
            let _ = self.caps.insert(cap);
        }
    }
}
