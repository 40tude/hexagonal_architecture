// Rust guideline compliant 2025-05-15
//! In-memory repository adapter for testing.

use domain::{Order, OrderError, OrderId, OrderRepository};
use std::collections::HashMap;

/// In-memory order repository for testing scenarios.
#[derive(Debug, Default)]
pub struct InMemoryOrderRepository {
    orders: HashMap<OrderId, Order>,
}

impl InMemoryOrderRepository {
    /// Creates a new empty in-memory repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("[InMemory] Saving order #{}", order.id);
        self.orders.insert(order.id, order.clone());
        Ok(())
    }

    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("[InMemory] Finding order #{id}");
        Ok(self.orders.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{LineItem, Money};

    #[test]
    fn save_and_find_order() {
        let mut repo = InMemoryOrderRepository::new();
        let order = Order::new(
            OrderId(1),
            vec![LineItem {
                name: "Test".to_string(),
                price: Money(100),
            }],
        )
        .unwrap();

        repo.save(&order).unwrap();
        let found = repo.find(OrderId(1)).unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, OrderId(1));
    }

    #[test]
    fn find_nonexistent_returns_none() {
        let repo = InMemoryOrderRepository::new();
        let found = repo.find(OrderId(999)).unwrap();

        assert!(found.is_none());
    }
}
