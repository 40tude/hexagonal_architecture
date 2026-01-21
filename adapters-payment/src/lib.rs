// =============================================================================
// ADAPTERS-PAYMENT CRATE - Processing Payments
// =============================================================================
//
// This crate provides concrete implementations of the `PaymentGateway` port.
// We have two adapters:
// - MockPaymentGateway: Always succeeds, perfect for testing
// - StripePaymentGateway: Simulates calling Stripe's API
//
// REAL-WORLD CONSIDERATIONS:
// --------------------------
// In a production app, this crate would have Cargo.toml dependencies like:
//
//     [dependencies]
//     domain = { path = "../domain" }
//     stripe-rust = "0.x"
//     tokio = { version = "1", features = ["rt-multi-thread"] }
//
// The adapters would be async and handle:
// - API authentication
// - Retry logic for transient failures
// - Rate limiting
// - Webhook validation
// - Error translation to domain errors
//
// Our simulated version shows the PATTERN without the complexity.

mod mock;
mod stripe;

pub use mock::MockPaymentGateway;
pub use stripe::StripePaymentGateway;
