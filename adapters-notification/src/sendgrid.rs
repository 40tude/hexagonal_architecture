// =============================================================================
// SendGrid Sender - Simulated Production Email Service
// =============================================================================
//
// This adapter simulates sending emails via SendGrid's API.
// In a real project, you'd add sendgrid-rs to Cargo.toml and make actual
// API calls here.
//
// The key point: the APPLICATION layer doesn't know this is SendGrid.
// It just knows it has something that implements `Sender`.

use domain::{Order, OrderError, Sender};

/// Simulated SendGrid notification sender.
///
/// In production, this would:
/// 1. Hold a SendGrid API client
/// 2. Format the email template
/// 3. Call the SendGrid API
/// 4. Handle rate limits and retries
///
/// Here we simulate it with println!
#[derive(Debug, Default, Clone, Copy)]
pub struct SendGridSender;

// In a real implementation, you'd have:
//
// pub struct SendGridSender {
//     api_key: String,
//     from_email: String,
// }
//
// impl SendGridSender {
//     pub fn new(api_key: &str, from_email: &str) -> Self {
//         Self {
//             api_key: api_key.to_string(),
//             from_email: from_email.to_string(),
//         }
//     }
// }

impl Sender for SendGridSender {
    /// Sends an order confirmation email via SendGrid.
    ///
    /// Real implementation would look like:
    /// ```ignore
    /// async fn send(&self, order: &Order) -> Result<(), OrderError> {
    ///     let message = Message::new()
    ///         .set_from(self.from_email.clone())
    ///         .set_subject(format!("Order #{} Confirmed", order.id))
    ///         .add_content(/* HTML template */);
    ///
    ///     sendgrid::send(&self.api_key, &message)
    ///         .await
    ///         .map_err(|_| OrderError::NotificationFailed)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Note how SendGrid errors become `OrderError::NotificationFailed`.
    /// The application layer never sees sendgrid::Error!
    fn send(&self, order: &Order) -> Result<(), OrderError> {
        println!(
            "  [SendGrid API] Sending email: 'Order #{} Confirmed'",
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
    fn sendgrid_sender_succeeds() {
        let sender = SendGridSender;
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
