// Rust guideline compliant 2025-05-15
//! Simulated `PostgreSQL` repository adapter.

use domain::{Order, OrderError, OrderId, OrderRepository};
use std::collections::HashMap;

/// Simulated `PostgreSQL` order repository.
///
/// In production, this would use an actual `PostgreSQL` connection.
#[derive(Debug, Default)]
pub struct PostgresOrderRepository {
    simulated_db: HashMap<OrderId, Order>,
}

impl PostgresOrderRepository {
    /// Creates a new simulated `PostgreSQL` repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl OrderRepository for PostgresOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("[Postgres] INSERT INTO orders VALUES ({}, ...)", order.id);
        self.simulated_db.insert(order.id, order.clone());
        Ok(())
    }

    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("[Postgres] SELECT * FROM orders WHERE id = {id}");
        Ok(self.simulated_db.get(&id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{LineItem, Money};

    #[test]
    fn postgres_save_and_find() {
        let mut repo = PostgresOrderRepository::new();
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
    }
}
