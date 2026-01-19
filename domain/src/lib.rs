// Rust guideline compliant 2025-05-15
//! Domain layer for hexagonal architecture.
//!
//! Contains core business logic: value objects, entities, and port traits.
//! No external dependencies - pure business rules.

use std::fmt;

// ============================================================================
// Value Objects
// ============================================================================

/// Unique identifier for an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u32);

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderId({})", self.0)
    }
}

/// Monetary value in cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(pub u32);

impl Money {
    /// Returns dollars portion.
    #[must_use]
    pub const fn dollars(self) -> u32 {
        self.0 / 100
    }

    /// Returns cents portion (0-99).
    #[must_use]
    pub const fn cents(self) -> u32 {
        self.0 % 100
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{:02}", self.dollars(), self.cents())
    }
}

// ============================================================================
// Entities
// ============================================================================

/// A single item in an order.
#[derive(Debug, Clone)]
pub struct LineItem {
    pub name: String,
    pub price: Money,
}

/// An order containing line items.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub items: Vec<LineItem>,
    pub total: Money,
}

impl Order {
    /// Creates a new order from line items.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::InvalidOrder`] if items is empty.
    pub fn new(id: OrderId, items: Vec<LineItem>) -> Result<Self, OrderError> {
        if items.is_empty() {
            return Err(OrderError::InvalidOrder);
        }

        let total = Money(items.iter().map(|item| item.price.0).sum());

        Ok(Self { id, items, total })
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Domain errors for order operations.
#[derive(Debug)]
pub enum OrderError {
    /// Order has no items.
    InvalidOrder,
    /// Payment processing failed.
    PaymentFailed,
    /// Storage operation failed.
    StorageFailed,
    /// Notification delivery failed.
    NotificationFailed,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOrder => write!(f, "InvalidOrder"),
            Self::PaymentFailed => write!(f, "PaymentFailed"),
            Self::StorageFailed => write!(f, "StorageFailed"),
            Self::NotificationFailed => write!(f, "NotificationFailed"),
        }
    }
}

impl std::error::Error for OrderError {}

// ============================================================================
// Port Traits (Output Ports)
// ============================================================================

/// Repository port for persisting orders.
pub trait OrderRepository {
    /// Saves an order to storage.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::StorageFailed`] if storage operation fails.
    fn save(&mut self, order: &Order) -> Result<(), OrderError>;

    /// Finds an order by ID.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::StorageFailed`] if retrieval fails.
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
}

/// Payment gateway port for processing payments.
pub trait PaymentGateway {
    /// Charges the given amount.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::PaymentFailed`] if payment fails.
    fn charge(&self, amount: Money) -> Result<(), OrderError>;
}

/// Notification service port for sending confirmations.
pub trait NotificationService {
    /// Sends order confirmation.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::NotificationFailed`] if notification fails.
    fn send_confirmation(&self, order: &Order) -> Result<(), OrderError>;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_new_with_items_succeeds() {
        let items = vec![
            LineItem {
                name: "Book".to_string(),
                price: Money(4999),
            },
            LineItem {
                name: "Pen".to_string(),
                price: Money(199),
            },
        ];

        let order = Order::new(OrderId(1), items).unwrap();

        assert_eq!(order.id, OrderId(1));
        assert_eq!(order.items.len(), 2);
        assert_eq!(order.total, Money(5198));
    }

    #[test]
    fn order_new_empty_items_fails() {
        let result = Order::new(OrderId(1), vec![]);

        assert!(matches!(result, Err(OrderError::InvalidOrder)));
    }

    #[test]
    fn money_display_formats_correctly() {
        assert_eq!(Money(4999).to_string(), "$49.99");
        assert_eq!(Money(100).to_string(), "$1.00");
        assert_eq!(Money(5).to_string(), "$0.05");
    }

    #[test]
    fn order_id_display_formats_correctly() {
        assert_eq!(OrderId(42).to_string(), "OrderId(42)");
    }
}
