// Rust guideline compliant 2025-05-15
//! Notification adapters implementing the `NotificationService` port.
//!
//! Provides concrete implementations for sending notifications.

mod console;
mod sendgrid;

pub use console::ConsoleNotificationService;
pub use sendgrid::SendGridNotificationService;
