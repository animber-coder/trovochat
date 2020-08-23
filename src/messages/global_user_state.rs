use crate::{irc::*, trovo::*, IntoOwned, MaybeOwned, Validator};

/// Sent on successful login, if **TAGS** capability have been sent beforehand.
#[derive(Clone, PartialEq)]
pub struct GlobalUserState<'a> {
    raw: MaybeOwned<'a>,
    tags: TagIndices,
    /// Your user-id
    pub user_id: MaybeOwned<'a>,
    /// Your display name, if set   
    pub display_name: Option<MaybeOwned<'a>>,
    /// Your color, if set. Defaults to `white`
    pub color: Color,
}

impl<'a> GlobalUserState<'a> {
    raw!();
    tags!();

    /// Your available emote sets, always contains atleast '0'
    pub fn emote_sets(&self) -> Vec<&str> {
        self.tags()
            .get("emote-sets")
            .map(|s| s.split(',').collect())
            .unwrap_or_else(|| vec!["0"])
    }

    /// Any badges you have
    pub fn badges(&self) -> Vec<Badge<'_>> {
        self.tags()
            .get("badges")
            .map(|s| s.split(',').filter_map(Badge::parse).collect())
            .unwrap_or_default()
    }

    /// Your user-id
    pub fn user_id(&self) -> &str {
        &*self.user_id
    }

    /// Your display name, if set   
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Your color, if set. Defaults to `white`
    pub fn color(&self) -> Color {
        self.color
    }
}

impl<'a> FromIrcMessage<'a> for GlobalUserState<'a> {
    type Error = MessageError;

    fn from_irc(msg: IrcMessage<'a>) -> Result<Self, Self::Error> {
        msg.expect_command(IrcMessage::GLOBAL_USER_STATE)?;

        let tag_index = msg.parse_tags();
        let tags = Tags {
            data: &msg.raw,
            indices: &tag_index,
        };

        let user_id = tags
            .get("user-id")
            .ok_or_else(|| MessageError::ExpectedTag {
                name: "user-id".to_string(),
            })
            .map(MaybeOwned::from)
            .map(MaybeOwned::into_owned)?;

        let display_name = tags
            .get("display-name")
            .map(MaybeOwned::from)
            .map(MaybeOwned::into_owned);

        let color = tags
            .get("color")
            .filter(|s| !s.is_empty())
            .map(std::str::FromStr::from_str)
            .transpose()
            .map_err(|err| MessageError::CannotParseTag {
                name: "color".into(),
                error: Box::new(err),
            })?
            .unwrap_or_default();

        let this = Self {
            user_id,
            display_name,
            color,
            tags: tag_index,
            raw: msg.raw,
        };

        Ok(this)
    }

    into_inner_raw!();
}

into_owned!(GlobalUserState {
    raw,
    tags,
    user_id,
    display_name,
    color,
});

impl_custom_debug!(GlobalUserState {
    raw,
    tags,
    emote_sets,
    badges,
    user_id,
    display_name,
    color,
});

serde_struct!(GlobalUserState {
    raw,
    tags,
    user_id,
    display_name,
    color,
    badges,
    emote_sets,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn global_user_state_serde() {
        let input = "@badge-info=;badges=;color=#FF69B4;display-name=shaken_bot;emote-sets=0;user-id=241015868;user-type= :tmi.trovo.tv GLOBALUSERSTATE\r\n";
        crate::serde::round_trip_json::<GlobalUserState>(input);
        crate::serde::round_trip_rmp::<GlobalUserState>(input);
    }

    #[test]
    fn global_user_state() {
        let input = "@badge-info=;badges=;color=#FF69B4;display-name=shaken_bot;emote-sets=0;user-id=241015868;user-type= :tmi.trovo.tv GLOBALUSERSTATE\r\n";
        for msg in parse(input).map(|s| s.unwrap()) {
            let msg = GlobalUserState::from_irc(msg).unwrap();

            assert_eq!(msg.user_id, "241015868");
            assert_eq!(msg.user_id(), "241015868");

            assert_eq!(msg.display_name.as_ref().unwrap(), "shaken_bot");
            assert_eq!(msg.display_name().unwrap(), "shaken_bot");

            let color = "#FF69B4".parse().unwrap();
            assert_eq!(msg.color, color);
            assert_eq!(msg.color(), color);

            assert_eq!(msg.emote_sets(), vec!["0"]);
        }
    }
}
