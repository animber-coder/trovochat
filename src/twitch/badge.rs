use std::borrow::Cow;

/// The kind of the [badges] that are associated with messages.
///
/// Any unknonw (e.g. custom badges/sub events, etc) are placed into the [Unknown] variant.
///
/// [badges]: ./struct.Badge.html
/// [Unknown]: ./enum.BadgeKind.html#variant.Unknown
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BadgeKind<'t> {
    /// Admin badge
    Admin,
    /// Bits badge
    Bits,
    /// Broadcaster badge
    Broadcaster,
    /// GlobalMod badge
    GlobalMod,
    /// Moderator badge
    Moderator,
    /// Subscriber badge
    Subscriber,
    /// Staff badge
    Staff,
    /// Turbo badge
    Turbo,
    /// Premium badge
    Premium,
    /// VIP badge
    VIP,
    /// Partner badge
    Partner,
    /// Unknown badge. Likely a custom badge
    Unknown(Cow<'t, str>),
}

/// Badges attached to a message
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Badge<'t> {
    /// The kind of the Badge
    pub kind: BadgeKind<'t>,
    /// Any associated data with the badge
    ///
    /// May be:
    /// - version
    /// - number of bits
    /// - number of months needed for sub badge
    /// - etc
    pub data: Cow<'t, str>,
}

impl<'t> Badge<'t> {
    /// Tries to parse a badge from this message part
    pub fn parse(input: &'t str) -> Option<Badge<'t>> {
        use BadgeKind::*;
        let mut iter = input.split('/');
        let kind = match iter.next()? {
            "admin" => Admin,
            "bits" => Bits,
            "broadcaster" => Broadcaster,
            "global_mod" => GlobalMod,
            "moderator" => Moderator,
            "subscriber" => Subscriber,
            "staff" => Staff,
            "turbo" => Turbo,
            "premium" => Premium,
            "vip" => VIP,
            "partner" => Partner,
            badge => Unknown(Cow::Borrowed(badge)),
        };
        iter.next().map(|data| Badge {
            kind,
            data: Cow::Borrowed(data),
        })
    }
}

/// Metadata to the chat badges
pub type BadgeInfo<'t> = Badge<'t>;

// TODO tests
