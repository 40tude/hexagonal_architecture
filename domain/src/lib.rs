// =============================================================================
// DOMAIN CRATE - The Sacred Core of the Application
// =============================================================================
//
// Welcome! This is the domain crate, the heart of hexagonal architecture.
// Everything else in this workspace depends on this crate, but this crate
// depends on NOTHING else (except std).
//
// In dip_06 (https://github.com/40tude/solid_test/tree/main/dip_06/src), domain
// was a module inside a single binary.
// Here, it's a full-fledged Rust CRATE. What's the difference?
//
// 1. ENFORCED ISOLATION: Cargo.toml has no internal dependencies.
//    We literally CAN'T import adapters here (they're not in dependencies)
//
// 2. INDEPENDENT COMPILATION: Change an adapter? Domain doesn't recompile.
//    This matters in large projects where build times add up.
//
// 3. PUBLISHABLE: We could publish this crate to crates.io independently.
//    Our domain logic becomes a reusable library!
//
// 4. EXPLICIT VERSIONING: Each crate can have its own version.
//    "Domain v2.0 with breaking changes" while adapters stay on v1.x.
//
// WHAT BELONGS HERE:
// ------------------
// - Value Objects (OrderId, Money)
// - Entities (Order, LineItem)
// - Domain Errors (OrderError)
// - Port Traits (OrderRepository, PaymentGateway, Sender)
//
// The port traits live here because the domain DEFINES what it needs.
// Adapters (in other crates) IMPLEMENT those needs.

use std::fmt;

// =============================================================================
// Value Objects
// =============================================================================
//
// Value objects are immutable types defined by their value, not identity.
// Two Money(100) are interchangeable, they represent the same thing.
//
// We use newtype wrappers (struct OrderId(u32)) instead of raw primitives.
// This gives us type safety: we can't pass a CustomerId where OrderId is expected.

/// A unique identifier for an order.
///
/// Using a newtype wrapper instead of raw `u32` provides type safety.
/// The compiler prevents mixing up different ID types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u32);

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderId({})", self.0)
    }
}

/// Monetary value in cents to avoid floating-point precision issues.
///
/// $49.99 is stored as `Money(4999)`. This is a common pattern in financial
/// applications (never use f64 for money!)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(pub u32);

impl Money {
    /// Returns the dollars portion (e.g., 49 for $49.99).
    #[must_use]
    pub const fn dollars(self) -> u32 {
        self.0 / 100
    }

    /// Returns the cents portion, 0-99 (e.g., 99 for $49.99).
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

// =============================================================================
// Entities
// =============================================================================
//
// Entities have identity: two orders with the same items are still different
// orders if they have different IDs. Unlike value objects, entities are mutable
// over their lifecycle (though we keep Order simple here).

/// A single item in an order.
#[derive(Debug, Clone)]
pub struct LineItem {
    pub name: String,
    pub price: Money,
}

/// An order containing line items.
///
/// Notice what's NOT here: database IDs, timestamps, "created_by" fields.
/// Those are infrastructure concerns. The domain only cares about what
/// an order IS from a business perspective.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub items: Vec<LineItem>,
    pub total: Money,
}

impl Order {
    /// Creates a new order from line items.
    ///
    /// This is where business rules live! "An order must have at least one item"
    /// is a business rule, not a database constraint or API validation.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::InvalidOrder`] if items is empty.
    pub fn new(id: OrderId, items: Vec<LineItem>) -> Result<Self, OrderError> {
        // Business rule: an order must have items
        if items.is_empty() {
            return Err(OrderError::InvalidOrder);
        }

        // Calculate total: pure business logic
        let total = Money(items.iter().map(|item| item.price.0).sum());

        Ok(Self { id, items, total })
    }
}

// =============================================================================
// Domain Errors
// =============================================================================
//
// These are BUSINESS errors, not technical errors. Notice:
// - "InvalidOrder" = business rule violation
// - "PaymentFailed" = business operation failed
//
// We DON'T have "DatabaseConnectionError" or "HttpTimeout".
// Those are adapter errors that get TRANSLATED into domain errors.
// The domain never sees sqlx::Error or reqwest::Error.
//
// More information available here:
// https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_02.html

/// Domain errors for order operations.
#[derive(Debug)]
pub enum OrderError {
    /// Order violates business rules (e.g., no items).
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

// =============================================================================
// Port Traits (Output Ports)
// =============================================================================
//
// Here's where it gets interesting! Remember the Sender trait from dip_02?
// That was our first "port". Ports define what the domain NEEDS from the
// outside world, without specifying HOW those needs are fulfilled.
//
// These traits live in the domain crate because:
// 1. The domain DEFINES its own contracts
// 2. Adapters (in other crates) IMPLEMENT these contracts
// 3. The dependency arrow points INWARD: adapters -> domain
//
// This is the Dependency Inversion Principle at the crate level!

/// Repository port for persisting orders.
///
/// The domain needs to store orders somewhere. It doesn't care if that
/// "somewhere" is PostgreSQL, MongoDB, a file, or a HashMap.
/// That's an adapter's decision.
pub trait OrderRepository {
    /// Saves an order to storage.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::StorageFailed`] if the operation fails.
    fn save(&mut self, order: &Order) -> Result<(), OrderError>;

    /// Finds an order by ID.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::StorageFailed`] if retrieval fails.
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
}

/// Payment gateway port for processing payments.
///
/// The domain needs to charge customers. It doesn't care if that's
/// via Stripe, PayPal, or carrier pigeons carrying gold coins.
pub trait PaymentGateway {
    /// Charges the given amount.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::PaymentFailed`] if payment fails.
    fn charge(&self, amount: Money) -> Result<(), OrderError>;
}

/// Notification port for sending messages to customers.
///
/// Hey, this is our old friend from dip_02! Same concept:
/// "I need to notify someone about an order."
/// Could be email, SMS, push notification, carrier pigeon...
pub trait Sender {
    /// Sends a notification about an order.
    ///
    /// # Errors
    ///
    /// Returns [`OrderError::NotificationFailed`] if sending fails.
    fn send(&self, order: &Order) -> Result<(), OrderError>;
}

// =============================================================================
// Tests
// =============================================================================
//
// Domain tests are PURE. No mocks needed. We're testing business logic
// with plain Rust values. This is one of the biggest benefits of a clean
// domain layer. Tests are simple and fast.

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
        assert_eq!(order.total, Money(5198)); // $51.98
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

// =============================================================================
// Key Takeaway
// =============================================================================
//
// This crate is the CENTER of our architecture. It:
// - Defines business entities and rules
// - Defines ports (traits) that describe external needs
// - Has ZERO dependencies on other workspace crates
// - Is pure, testable, and reusable
//
// Everything else depends on this crate. This crate depends on nothing.
// That's the essence of hexagonal architecture!
//
// Next: check out the `application` crate to see how OrderService uses these ports.
