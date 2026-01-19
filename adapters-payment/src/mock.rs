// Rust guideline compliant 2025-05-15
//! Mock payment gateway for testing.

use domain::{Money, OrderError, PaymentGateway};

/// Mock payment gateway that always succeeds.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockPaymentGateway;

impl PaymentGateway for MockPaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!("[Mock] Charging {amount}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_charge_succeeds() {
        let gateway = MockPaymentGateway;
        let result = gateway.charge(Money(1000));

        assert!(result.is_ok());
    }
}
