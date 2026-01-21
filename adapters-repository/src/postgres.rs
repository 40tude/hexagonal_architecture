// =============================================================================
// PostgreSQL Repository - Simulated Production Database
// =============================================================================
//
// This adapter simulates a PostgreSQL database.
// In production, you'd add sqlx or diesel to Cargo.toml and execute real queries.
//
// ERROR TRANSLATION:
// ------------------
// A key responsibility of adapters is translating external errors to domain errors.
// sqlx::Error -> OrderError::StorageFailed
//
// The application layer never sees database-specific errors!

use domain::{Order, OrderError, OrderId, OrderRepository};
use std::collections::HashMap;

/// Simulated PostgreSQL order repository.
///
/// In production, this would hold a connection pool:
/// ```ignore
/// pub struct PostgresOrderRepository {
///     pool: sqlx::PgPool,
/// }
/// ```
#[derive(Debug, Default)]
pub struct PostgresOrderRepository {
    // In reality: pool: sqlx::PgPool
    // For demo: just a HashMap
    simulated_db: HashMap<OrderId, Order>,
}

impl PostgresOrderRepository {
    /// Creates a new simulated PostgreSQL repository.
    ///
    /// Real implementation:
    /// ```ignore
    /// pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
    ///     let pool = PgPool::connect(database_url).await?;
    ///     Ok(Self { pool })
    /// }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl OrderRepository for PostgresOrderRepository {
    /// Saves an order to PostgreSQL.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn save(&mut self, order: &Order) -> Result<(), OrderError> {
    ///     sqlx::query(
    ///         "INSERT INTO orders (id, total) VALUES ($1, $2)"
    ///     )
    ///     .bind(order.id.0)
    ///     .bind(order.total.0)
    ///     .execute(&self.pool)
    ///     .await
    ///     .map_err(|e| {
    ///         // Log the actual error for debugging
    ///         tracing::error!("Database error: {e}");
    ///         // Return domain error to application
    ///         OrderError::StorageFailed
    ///     })?;
    ///     Ok(())
    /// }
    /// ```
    fn save(&mut self, order: &Order) -> Result<(), OrderError> {
        println!("  [Postgres] INSERT INTO orders VALUES ({}, ...)", order.id);
        self.simulated_db.insert(order.id, order.clone());
        Ok(())
    }

    /// Retrieves an order from PostgreSQL.
    ///
    /// Real implementation:
    /// ```ignore
    /// async fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
    ///     let row = sqlx::query_as::<_, OrderRow>(
    ///         "SELECT * FROM orders WHERE id = $1"
    ///     )
    ///     .bind(id.0)
    ///     .fetch_optional(&self.pool)
    ///     .await
    ///     .map_err(|_| OrderError::StorageFailed)?;
    ///
    ///     Ok(row.map(Into::into))
    /// }
    /// ```
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        println!("  [Postgres] SELECT * FROM orders WHERE id = {id}");
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
