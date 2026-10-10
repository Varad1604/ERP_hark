//! Seed of the Finance money model (ADR-L5-0032): fee lines are exact `rust_decimal` amounts;
//! rounding is an explicit, policy-bound step rather than an implicit default.

use rust_decimal::{Decimal, RoundingStrategy};
use sutra_kernel::money::{CurrencyCode, Money, MoneyError};

/// One line of a fee demand.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeeLine {
    pub label: String,
    pub amount: Money,
    /// Fractional concession, e.g. `0.25` for 25 %.
    #[serde(with = "rust_decimal::serde::str")]
    #[schema(value_type = String, example = "0.25")]
    pub concession: Decimal,
}

/// Rounding policy bound by the authoritative fee rule (here: minor units + banker's rounding).
#[derive(Debug, Clone, Copy)]
pub struct RoundingPolicy {
    pub scale: u32,
    pub strategy: RoundingStrategy,
}

/// Net payable = Σ amount × (1 − concession), rounded once at the end with `policy`.
pub fn net_payable(
    currency: CurrencyCode,
    lines: &[FeeLine],
    policy: RoundingPolicy,
) -> Result<Money, MoneyError> {
    let nets = lines
        .iter()
        .map(|l| l.amount.checked_mul(Decimal::ONE - l.concession))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Money::sum(currency, nets)?.round(policy.scale, policy.strategy))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn computes_exact_net_with_single_rounding() {
        let lines = vec![
            FeeLine {
                label: "Tuition".into(),
                amount: Money::new(d("85000.00"), CurrencyCode::INR),
                concession: d("0.25"),
            },
            FeeLine {
                label: "Lab".into(),
                amount: Money::new(d("3333.33"), CurrencyCode::INR),
                concession: d("0.10"),
            },
        ];
        let policy = RoundingPolicy {
            scale: 2,
            strategy: RoundingStrategy::MidpointNearestEven,
        };
        // 63750 + 2999.997 = 66749.997 -> 66750.00
        assert_eq!(
            net_payable(CurrencyCode::INR, &lines, policy)
                .unwrap()
                .amount,
            d("66750.00")
        );
    }
}
