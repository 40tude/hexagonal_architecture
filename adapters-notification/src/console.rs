// Rust guideline compliant 2025-05-15
//! Console notification service for testing.

use domain::{NotificationService, Order, OrderError};

/// Console-based notification service for testing.
#[derive(Debug, Default, Clone, Copy)]
pub struct ConsoleNotificationService;

impl NotificationService for ConsoleNotificationService {
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "[Console] Order #{} confirmed - Total: {}",
            order.id, order.total
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{LineItem, Money, OrderId};

    #[test]
    fn console_notification_succeeds() {
        let service = ConsoleNotificationService;
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
