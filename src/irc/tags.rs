#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

/// Tags are IRCv3 message tags. Trovo uses them extensively
#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tags(pub(crate) HashMap<String, String>);

impl Tags {
    pub(super) fn parse(input: &str) -> Self {
        debug_assert!(input.starts_with('@'));
        let map = input[1..].split_terminator(';').filter_map(|p| {
            let pos = p.find('=')?;
            Some((p[..pos].to_string(), p[pos + 1..].to_string()))
        });
        Self(map.collect())
    }

    /// Clones the inner HashMap (for reuse elsewhere..)
    pub fn inner_clone(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}
