// =============================================================================
// In-Memory Repository - A HashMap Pretending to Be a Database
// =============================================================================
//
// This is the simplest possible implementation of OrderRepository.
// A HashMap is our "database". When the process exits, data is gone.
// And that's perfectly fine for testing!
//
// WHY USE THIS?
// -------------
// 1. Unit tests run fast: no database setup
// 2. CI/CD pipelines don't need database containers
// 3. Local development works without infrastructure
// 4. Demos work anywhere

use domain::{Order, OrderError, OrderId, OrderRepository};
use std::collections::HashMap;

/// In-memory order repository for testing scenarios.
///
/// Uses a HashMap as its "database". Data is lost when the process exits.
/// Perfect for tests, development, and demos.
#[derive(Debug, Default)]
pub struct InMemoryOrderRepository {
    orders: HashMap<OrderId, Order>,
}

impl InMemoryOrderRepository {
    /// Creates a new empty in-memory repository.
    ///
    /// Compare this to PostgresOrderRepository::new() which would take
    /// a connection string. Here? Nothing needed. Just an empty HashMap.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl OrderRepository for InMemoryOrderRepository {
    /// Saves an order to the HashMap.
    ///
    /// In PostgreSQL: `INSERT INTO orders (...) VALUES (...)`
    /// Here: `HashMap.insert()`
    ///
    /// The application layer doesn't know the difference!
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("  [InMemory] Saving order #{}", order.id);
        self.orders.insert(order.id, order.clone());
        Ok(())
    }

    /// Finds an order by ID.
    ///
    /// In PostgreSQL: `SELECT * FROM orders WHERE id = $1`
    /// Here: `HashMap.get()`
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("  [InMemory] Finding order #{id}");
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
