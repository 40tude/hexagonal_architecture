// Rust guideline compliant 2025-05-15
//! Application layer for hexagonal architecture.
//!
//! Contains use cases that orchestrate domain logic through port abstractions.
//! Depends only on the domain layer - no infrastructure knowledge.

use domain::{
    LineItem, NotificationService, Order, OrderError, OrderId, OrderRepository, PaymentGateway,
};

/// Application service for order operations.
///
/// Orchestrates domain logic using injected port implementations.
/// Generic over repository, payment, and notification adapters.
#[derive(Debug)]
pub struct OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: NotificationService,
{
    repository: R,
    payment: P,
    notifications: N,
    next_id: u32,
}

impl<R, P, N> OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: NotificationService,
{
    /// Creates a new order service with injected dependencies.
    pub fn new(repository: R, payment: P, notifications: N) -> Self {
        Self {
            repository,
            payment,
            notifications,
            next_id: 1,
        }
    }

    /// Places a new order.
    ///
    /// Processes payment, persists order, and sends confirmation.
    ///
    /// # Errors
    ///
    /// Returns error if any step fails (payment, storage, notification).
    pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
        let order_id = OrderId(self.next_id);
        self.next_id += 1;

        let order = Order::new(order_id, items)?;

        self.payment.charge(order.total)?;
        self.repository.save(&order)?;
        self.notifications.send_confirmation(&order)?;

        Ok(order)
    }

    /// Retrieves an order by ID.
    ///
    /// # Errors
    ///
    /// Returns error if retrieval fails.
    pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        self.repository.find(id)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Money;
    use std::cell::RefCell;
    use std::collections::HashMap;

    // Test doubles
    struct MockRepository {
        orders: RefCell<HashMap<OrderId, Order>>,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                orders: RefCell::new(HashMap::new()),
            }
        }
    }

    impl OrderRepository for MockRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError> {
            self.orders.borrow_mut().insert(order.id, order.clone());
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            Ok(self.orders.borrow().get(&id).cloned())
        }
    }

    struct MockPayment;

    impl PaymentGateway for MockPayment {
        fn charge(&self, _amount: Money) -> Result<(), OrderError> {
            Ok(())
        }
    }

    struct MockNotification;

    impl NotificationService for MockNotification {
        fn send_confirmation(&self, _order: &Order) -> Result<(), OrderError> {
            Ok(())
        }
    }

    struct FailingPayment;

    impl PaymentGateway for FailingPayment {
        fn charge(&self, _amount: Money) -> Result<(), OrderError> {
            Err(OrderError::PaymentFailed)
        }
    }

    #[test]
    fn place_order_succeeds() {
        let mut service = OrderService::new(MockRepository::new(), MockPayment, MockNotification);

        let items = vec![LineItem {
            name: "Test".to_string(),
            price: Money(1000),
        }];

        let result = service.place_order(items);

        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.id, OrderId(1));
        assert_eq!(order.total, Money(1000));
    }

    #[test]
    fn place_order_payment_fails() {
        let mut service = OrderService::new(MockRepository::new(), FailingPayment, MockNotification);

        let items = vec![LineItem {
            name: "Test".to_string(),
            price: Money(1000),
        }];

        let result = service.place_order(items);

        assert!(matches!(result, Err(OrderError::PaymentFailed)));
    }

    #[test]
    fn get_order_returns_saved_order() {
        let mut service = OrderService::new(MockRepository::new(), MockPayment, MockNotification);

        let items = vec![LineItem {
            name: "Test".to_string(),
            price: Money(1000),
        }];

        let order = service.place_order(items).unwrap();
        let retrieved = service.get_order(order.id).unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, order.id);
    }
}
