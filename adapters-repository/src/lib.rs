// =============================================================================
// ADAPTERS-REPOSITORY CRATE - Persisting Orders
// =============================================================================
//
// This crate provides concrete implementations of the `OrderRepository` port.
// We have two adapters:
// - InMemoryOrderRepository: HashMap-based, perfect for testing
// - PostgresOrderRepository: Simulates a real database
//
// THE REPOSITORY PATTERN:
// -----------------------
// A repository abstracts data storage. The application says "save this order"
// without knowing if it's going to PostgreSQL, MongoDB, a file, or memory.
//
// This is one of the oldest and most useful patterns in software architecture.
// It dates back to Eric Evans' Domain-Driven Design (2003) and earlier.
//
// REAL-WORLD CONSIDERATIONS:
// --------------------------
// Production repositories handle:
// - Connection pooling (sqlx::PgPool)
// - Transactions
// - Optimistic locking
// - Query building
// - Error translation
//
// Our simulated version shows the pattern without the complexity.

mod in_memory;
mod postgres;

pub use in_memory::InMemoryOrderRepository;
pub use postgres::PostgresOrderRepository;
