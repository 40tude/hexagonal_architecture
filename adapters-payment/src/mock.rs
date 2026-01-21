// =============================================================================
// Mock Payment Gateway - For Testing
// =============================================================================
//
// This adapter always succeeds. It's the "happy path" mock.
// Perfect for testing the normal flow of your application.
//
// In a more sophisticated test setup, you might also have:
// - FailingPaymentGateway (always fails)
// - FlakeyPaymentGateway (fails randomly - for chaos testing)
// - SlowPaymentGateway (adds delays - for timeout testing)
//
// Each helps test different scenarios without real payment APIs.

use domain::{Money, OrderError, PaymentGateway};

/// Mock payment gateway that always succeeds.
///
/// No real money moves. No API calls. Just a log line.
/// But from OrderService's perspective, the contract is fulfilled!
#[derive(Debug, Default, Clone, Copy)]
pub struct MockPaymentGateway;

impl PaymentGateway for MockPaymentGateway {
    /// "Charges" the amount by printing to stdout.
    ///
    /// Returns Ok(()) always - the happy path.
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!("  [Mock] Charging {amount}");
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
