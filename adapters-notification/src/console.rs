// =============================================================================
// Console Sender - For Testing and Development
// =============================================================================
//
// This adapter "sends" notifications by printing to stdout.
// Perfect for testing and local development - no email server needed!
//
// Remember the Email struct from dip_02? This is its spiritual successor.
// Same concept: implement the Sender trait with a simple implementation.

use domain::{Order, OrderError, Sender};

/// Console-based notification sender for testing.
///
/// "Sends" notifications by printing to the console.
/// No network calls, no external services - just println!
#[derive(Debug, Default, Clone, Copy)]
pub struct ConsoleSender;

impl Sender for ConsoleSender {
    /// "Sends" a notification by printing to stdout.
    ///
    /// In production, this might call SendGrid, queue a message in RabbitMQ,
    /// or send an SMS via Twilio. Here, it just prints. And that's enough
    /// for testing!
    fn send(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "  [Console] Order #{} confirmed! Total: {}",
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
    fn console_sender_succeeds() {
        let sender = ConsoleSender;
        let order = Order::new(
            OrderId(1),
            vec![LineItem {
                name: "Test".to_string(),
                price: Money(100),
            }],
        )
        .unwrap();

        let result = sender.send(&order);

        assert!(result.is_ok());
    }
}
