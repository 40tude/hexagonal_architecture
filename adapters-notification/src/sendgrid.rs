// Rust guideline compliant 2025-05-15
//! Simulated `SendGrid` notification service.

use domain::{NotificationService, Order, OrderError};

/// Simulated `SendGrid` notification service.
///
/// In production, this would use the actual `SendGrid` API.
#[derive(Debug, Default, Clone, Copy)]
pub struct SendGridNotificationService;

impl NotificationService for SendGridNotificationService {
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "[SendGrid API] POST /mail/send to=customer@example.com subject='Order #{} Confirmed'",
            order.id
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{LineItem, Money, OrderId};

    #[test]
    fn sendgrid_notification_succeeds() {
        let service = SendGridNotificationService;
        let order = Order::new(
            OrderId(1),
            vec![LineItem {
                name: "Test".to_string(),
                price: Money(100),
            }],
        )
        .unwrap();

        let result = service.send_confirmation(&order);

        assert!(result.is_ok());
    }
}
