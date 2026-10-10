//! Monetary representation (ADR-L5-0032).
//!
//! `rust_decimal::Decimal` is the authoritative monetary scalar. No global precision, scale or
//! rounding rule is introduced here: each calculation binds its own authoritative policy, which is
//! why rounding is an explicit argument rather than an implicit default. Amounts serialize as
//! JSON strings to avoid binary floating point on the wire.

use rust_decimal::{Decimal, RoundingStrategy};
use std::fmt;

/// ISO 4217 alphabetic currency code (e.g. `INR`).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(try_from = "String", into = "String")]
#[schema(value_type = String, example = "INR")]
pub struct CurrencyCode([u8; 3]);

impl CurrencyCode {
    pub const INR: CurrencyCode = CurrencyCode(*b"INR");

    pub fn as_str(&self) -> &str {
        // Invariant: always three ASCII uppercase letters.
        std::str::from_utf8(&self.0).unwrap_or("XXX")
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let b = s.as_bytes();
        if b.len() == 3 && b.iter().all(u8::is_ascii_uppercase) {
            Ok(Self([b[0], b[1], b[2]]))
        } else {
            Err(format!("invalid ISO 4217 currency code: {s}"))
        }
    }
}

impl From<CurrencyCode> for String {
    fn from(c: CurrencyCode) -> Self {
        c.as_str().to_owned()
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An exact monetary amount in a specific currency.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    #[serde(with = "rust_decimal::serde::str")]
    #[schema(value_type = String, example = "1250.50")]
    pub amount: Decimal,
    pub currency: CurrencyCode,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MoneyError {
    #[error("currency mismatch: {0} vs {1}")]
    CurrencyMismatch(CurrencyCode, CurrencyCode),
    #[error("arithmetic overflow")]
    Overflow,
}

impl Money {
    pub fn new(amount: Decimal, currency: CurrencyCode) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: CurrencyCode) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn checked_add(self, other: Money) -> Result<Money, MoneyError> {
        self.same_currency(&other)?;
        self.amount
            .checked_add(other.amount)
            .map(|amount| Money {
                amount,
                currency: self.currency,
            })
            .ok_or(MoneyError::Overflow)
    }

    pub fn checked_sub(self, other: Money) -> Result<Money, MoneyError> {
        self.same_currency(&other)?;
        self.amount
            .checked_sub(other.amount)
            .map(|amount| Money {
                amount,
                currency: self.currency,
            })
            .ok_or(MoneyError::Overflow)
    }

    pub fn checked_mul(self, factor: Decimal) -> Result<Money, MoneyError> {
        self.amount
            .checked_mul(factor)
            .map(|amount| Money {
                amount,
                currency: self.currency,
            })
            .ok_or(MoneyError::Overflow)
    }

    /// Rounds to `scale` decimal places with the caller-supplied, policy-bound strategy.
    pub fn round(self, scale: u32, strategy: RoundingStrategy) -> Money {
        Money {
            amount: self.amount.round_dp_with_strategy(scale, strategy),
            currency: self.currency,
        }
    }

    /// Sums an iterator of amounts that must all share `currency`.
    pub fn sum<I: IntoIterator<Item = Money>>(
        currency: CurrencyCode,
        items: I,
    ) -> Result<Money, MoneyError> {
        items
            .into_iter()
            .try_fold(Money::zero(currency), Money::checked_add)
    }

    fn same_currency(&self, other: &Money) -> Result<(), MoneyError> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(MoneyError::CurrencyMismatch(self.currency, other.currency))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap_or_default()
    }

    #[test]
    fn exact_base10_arithmetic() {
        let a = Money::new(d("0.10"), CurrencyCode::INR);
        let b = Money::new(d("0.20"), CurrencyCode::INR);
        assert_eq!(a.checked_add(b).map(|m| m.amount), Ok(d("0.30")));
    }

    #[test]
    fn currency_mismatch_is_rejected() {
        let usd = CurrencyCode::try_from("USD".to_string()).unwrap_or(CurrencyCode::INR);
        let r = Money::new(d("1"), CurrencyCode::INR).checked_add(Money::new(d("1"), usd));
        assert!(matches!(r, Err(MoneyError::CurrencyMismatch(_, _))));
    }

    #[test]
    fn rounding_is_explicit() {
        let m = Money::new(d("10.125"), CurrencyCode::INR);
        assert_eq!(
            m.round(2, RoundingStrategy::MidpointNearestEven).amount,
            d("10.12")
        );
        assert_eq!(
            m.round(2, RoundingStrategy::MidpointAwayFromZero).amount,
            d("10.13")
        );
    }

    #[test]
    fn serializes_amount_as_string() {
        let m = Money::new(d("1250.50"), CurrencyCode::INR);
        let json = serde_json::to_value(m).unwrap_or_default();
        assert_eq!(
            json,
            serde_json::json!({"amount": "1250.50", "currency": "INR"})
        );
        let back: Money = serde_json::from_value(json).expect("roundtrip");
        assert_eq!(back, m);
    }

    #[test]
    fn rejects_bad_currency() {
        assert!(CurrencyCode::try_from("inr".to_string()).is_err());
        assert!(CurrencyCode::try_from("RUPEE".to_string()).is_err());
    }

    #[test]
    fn sums_lines() {
        let lines = ["100.00", "250.50", "0.25"].map(|s| Money::new(d(s), CurrencyCode::INR));
        assert_eq!(
            Money::sum(CurrencyCode::INR, lines).map(|m| m.amount),
            Ok(d("350.75"))
        );
    }
}
