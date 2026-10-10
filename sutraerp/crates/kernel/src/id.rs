//! UUIDv7 technical identifiers (ADR-L5-0036 §3.3).
//!
//! Technical ids are *not* business identifiers and their embedded timestamp must never be used
//! as an authoritative business time.

use uuid::Uuid;

/// Generates a new UUIDv7.
pub fn new_v7() -> Uuid {
    Uuid::now_v7()
}

/// Declares a strongly typed UUIDv7 identifier newtype.
#[macro_export]
macro_rules! typed_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Serialize, serde::Deserialize, utoipa::ToSchema,
        )]
        #[serde(transparent)]
        #[schema(value_type = String, format = Uuid)]
        pub struct $name(pub uuid::Uuid);

        impl $name {
            /// Allocates a fresh UUIDv7 identifier.
            pub fn new() -> Self { Self($crate::id::new_v7()) }
            pub fn as_uuid(&self) -> uuid::Uuid { self.0 }
        }

        impl Default for $name {
            fn default() -> Self { Self::new() }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
        }

        impl std::str::FromStr for $name {
            type Err = uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> { uuid::Uuid::parse_str(s).map(Self) }
        }

        impl From<uuid::Uuid> for $name {
            fn from(u: uuid::Uuid) -> Self { Self(u) }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    typed_id!(
        /// Test id.
        SampleId
    );

    #[test]
    fn generates_version_7() {
        let id = new_v7();
        assert_eq!(id.get_version_num(), 7);
        assert_eq!(SampleId::new().as_uuid().get_version_num(), 7);
    }

    #[test]
    fn v7_ids_are_monotonic_in_practice() {
        let a = new_v7();
        let b = new_v7();
        assert!(a < b);
    }

    #[test]
    fn typed_id_roundtrips_as_plain_string() {
        let id = SampleId::new();
        let json = serde_json::to_string(&id).unwrap_or_default();
        assert_eq!(json, format!("\"{id}\""));
        let back: SampleId = id.to_string().parse().unwrap_or_default();
        assert_eq!(back, id);
    }
}
