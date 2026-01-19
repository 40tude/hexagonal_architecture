// Rust guideline compliant 2025-05-15
//! Repository adapters implementing the `OrderRepository` port.
//!
//! Provides concrete implementations for order persistence.

mod in_memory;
mod postgres;

pub use in_memory::InMemoryOrderRepository;
pub use postgres::PostgresOrderRepository;
