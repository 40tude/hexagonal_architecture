// Rust guideline compliant 2025-05-15
//! Payment adapters implementing the `PaymentGateway` port.
//!
//! Provides concrete implementations for payment processing.

mod mock;
mod stripe;

pub use mock::MockPaymentGateway;
pub use stripe::StripePaymentGateway;
