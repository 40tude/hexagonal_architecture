// =============================================================================
// Stripe Payment Gateway - Simulated Production Service
// =============================================================================
//
// This adapter simulates calling Stripe's payment API.
// In production, we'd add stripe-rust to Cargo.toml and make real API calls.
//
// The adapter's job is to TRANSLATE between:
// - Domain concepts (Money, OrderError)
// - External API concepts (stripe::Amount, stripe::Error)

use domain::{Money, OrderError, PaymentGateway};

/// Simulated Stripe payment gateway.
///
/// In production, this would:
/// 1. Hold a Stripe API client with secret key
/// 2. Create a PaymentIntent or Charge
/// 3. Handle 3D Secure if needed
/// 4. Deal with webhooks for async confirmation
/// 5. Translate Stripe errors to domain errors
#[derive(Debug, Default, Clone, Copy)]
pub struct StripePaymentGateway;

// In a real implementation:
//
// pub struct StripePaymentGateway {
//     client: stripe::Client,
// }
//
// impl StripePaymentGateway {
//     pub fn new(secret_key: &str) -> Self {
//         Self {
//             client: stripe::Client::new(secret_key),
//         }
//     }
// }

impl PaymentGateway for StripePaymentGateway {
    /// Charges a customer via Stripe.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn charge(&self, amount: Money) -> Result<(), OrderError> {
    ///     let charge = CreateCharge {
    ///         amount: amount.0 as i64,  // Stripe uses cents too!
    ///         currency: "usd",
    ///         source: "tok_visa",  // Token from frontend
    ///         ..Default::default()
    ///     };
    ///
    ///     self.client
    ///         .charges()
    ///         .create(charge)
    ///         .await
    ///         .map_err(|_| OrderError::PaymentFailed)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Note: Stripe errors become `OrderError::PaymentFailed`.
    /// The application layer never sees stripe::Error!
    fn charge(&self, amount: Money) -> Result<(), OrderError> {
        println!("  [Stripe API] POST /charges amount={amount}");
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
