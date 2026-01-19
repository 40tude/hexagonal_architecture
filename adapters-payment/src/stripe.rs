// Rust guideline compliant 2025-05-15
//! Simulated Stripe payment gateway.

use domain::{Money, OrderError, PaymentGateway};

/// Simulated Stripe payment gateway.
///
/// In production, this would use the actual Stripe API.
#[derive(Debug, Default, Clone, Copy)]
pub struct StripePaymentGateway;

impl PaymentGateway for StripePaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!("[Stripe API] POST /charges amount={amount}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_charge_succeeds() {
        let gateway = StripePaymentGateway;
        let result = gateway.charge(Money(5000));

        assert!(result.is_ok());
    }
}
