//! Optimistic concurrency (ADR-L5-0006 §7).
//!
//! Every mutable aggregate row carries a monotonically increasing `version` column. Writers
//! state the version they read; a mismatch is an explicit, detectable conflict.

use crate::error::KernelError;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(transparent)]
pub struct Version(pub i64);

impl Version {
    pub const INITIAL: Version = Version(1);

    pub fn next(self) -> Version {
        Version(self.0 + 1)
    }

    /// Checks the caller's expected version against the stored one.
    pub fn expect(self, expected: Version) -> Result<(), KernelError> {
        if self == expected {
            Ok(())
        } else {
            Err(KernelError::VersionConflict {
                expected: expected.0,
                actual: self.0,
            })
        }
    }

    /// Renders as a strong HTTP entity tag (`"7"`).
    pub fn to_etag(self) -> String {
        format!("\"{}\"", self.0)
    }

    /// Parses an `If-Match` value produced by [`Version::to_etag`] (weak tags are rejected).
    pub fn from_if_match(raw: &str) -> Option<Version> {
        let raw = raw.trim();
        let inner = raw.strip_prefix('"')?.strip_suffix('"')?;
        inner.parse::<i64>().ok().filter(|v| *v >= 1).map(Version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_conflict() {
        assert!(Version(3).expect(Version(3)).is_ok());
        assert!(matches!(
            Version(4).expect(Version(3)),
            Err(KernelError::VersionConflict {
                expected: 3,
                actual: 4
            })
        ));
    }

    #[test]
    fn etag_roundtrip() {
        let v = Version(42);
        assert_eq!(Version::from_if_match(&v.to_etag()), Some(v));
        assert_eq!(Version::from_if_match("W/\"42\""), None);
        assert_eq!(Version::from_if_match("\"0\""), None);
        assert_eq!(Version::from_if_match("42"), None);
    }
}
