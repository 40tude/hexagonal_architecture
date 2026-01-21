// =============================================================================
// APPLICATION CRATE - The Orchestrator
// =============================================================================
//
// Welcome to the application layer! This crate sits between the domain
// and the adapters. It orchestrates USE CASES, the things your application
// actually DOES.
//
// In dip_02, OrderService was inside the domain module.
// In dip_06, it moved to a separate application module.
// Here, it's a full CRATE. Same pattern, increasing isolation.
//
// WHAT'S THE DIFFERENCE FROM DOMAIN?
// ----------------------------------
// Domain = WHAT things ARE (Order, Money, business rules)
// Application = WHAT HAPPENS (place order, get order, use cases)
//
// Think restaurant analogy:
// - Domain = recipes, ingredients, cooking techniques
// - Application = the head chef orchestrating dinner service
//
// The chef doesn't invent new recipes during service. They follow recipes
// (domain rules) and coordinate the kitchen (call adapters through ports).
//
// DEPENDENCY RULE:
// ----------------
// Look at Cargo.toml: we depend ONLY on `domain`.
// No adapter crates! We don't know if we're using PostgreSQL or a HashMap.
// We just know we have something that implements OrderRepository.

use domain::{LineItem, Order, OrderError, OrderId, OrderRepository, PaymentGateway, Sender};

// =============================================================================
// Order Service - The Use Case Handler
// =============================================================================
//
// This struct is generic over THREE type parameters: R, P, N.
// Each is constrained by a port trait from the domain crate.
//
// In dip_02, we had: OrderService<S: Sender>
// Now we have:       OrderService<R: OrderRepository, P: PaymentGateway, N: Sender>
//
// Same pattern, just scaled up. More dependencies, more flexibility.

/// Application service for order operations.
///
/// Orchestrates domain logic using injected port implementations.
/// This is where use cases live: the "what happens when" of your app.
///
/// Generic over:
/// - `R`: Repository adapter (where orders are stored)
/// - `P`: Payment adapter (how payments are processed)
/// - `N`: Notification adapter (how customers are notified)
#[derive(Debug)]
pub struct OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: Sender,
{
    // These fields hold our adapters, but we only know them by their traits!
    // We don't know if `repository` is PostgreSQL or InMemory.
    // We don't care! That's abstraction at work.
    repository: R,
    payment: P,
    sender: N,

    // Application state - not business logic.
    // In a real app, IDs would come from the database or UUID generator.
    next_id: u32,
}

impl<R, P, N> OrderService<R, P, N>
where
    R: OrderRepository,
    P: PaymentGateway,
    N: Sender,
{
    /// Creates a new order service with injected dependencies.
    ///
    /// This is Dependency Injection! The caller (main.rs in the app crate)
    /// decides which concrete implementations to use. We just accept anything
    /// that implements the required traits.
    ///
    /// # Why This Matters
    /// - Testing: pass mock adapters, no real database needed
    /// - Flexibility: swap PostgreSQL for MongoDB without changing this code
    /// - Clarity: dependencies are explicit in the function signature
    pub fn new(repository: R, payment: P, sender: N) -> Self {
        Self {
            repository,
            payment,
            sender,
            next_id: 1,
        }
    }

    /// Places a new order - the main use case.
    ///
    /// Look at what this method does:
    /// 1. Generate an ID (application concern)
    /// 2. Create the Order (delegates to domain)
    /// 3. Charge payment (calls port -> adapter)
    /// 4. Save order (calls port -> adapter)
    /// 5. Send notification (calls port -> adapter)
    ///
    /// The ORDER of operations matters! That's orchestration.
    /// We charge before saving because we don't want to save unpaid orders.
    ///
    /// # Errors
    ///
    /// Returns error if any step fails (validation, payment, storage, notification).
    pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
        // Step 1: Generate ID (application layer responsibility)
        let order_id = OrderId(self.next_id);
        self.next_id += 1;

        // Step 2: Create order using domain logic
        // Order::new() enforces business rules
        let order = Order::new(order_id, items)?;

        // Steps 3-5: Orchestrate external operations
        // Each call goes through a port to an adapter.
        // We don't know what adapter and we don't care!
        self.payment.charge(order.total)?;
        self.repository.save(&order)?;
        self.sender.send(&order)?;

        Ok(order)
    }

    /// Retrieves an order by ID.
    ///
    /// A simple use case: just delegate to the repository.
    ///
    /// # Errors
    ///
    /// Returns error if retrieval fails.
    pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
        self.repository.find(id)
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// Application tests need mock adapters because we're testing orchestration.
// We create simple test doubles that implement the port traits.
// No real database, no real payment API - just verifying the flow works.

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Money;
    use std::cell::RefCell;
    use std::collections::HashMap;

    // -------------------------------------------------------------------------
    // Test Doubles (Mock Adapters)
    // -------------------------------------------------------------------------
    // These are minimal implementations just for testing.
    // They prove that our application layer works with ANY implementation
    // of the port traits.

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

    struct MockSender;

    impl Sender for MockSender {
        fn send(&self, _order: &Order) -> Result<(), OrderError> {
            Ok(())
        }
    }

    struct FailingPayment;

    impl PaymentGateway for FailingPayment {
        fn charge(&self, _amount: Money) -> Result<(), OrderError> {
            Err(OrderError::PaymentFailed)
        }
    }

    // -------------------------------------------------------------------------
    // Actual Tests
    // -------------------------------------------------------------------------

    #[test]
    fn place_order_succeeds() {
        let mut service = OrderService::new(MockRepository::new(), MockPayment, MockSender);

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
        // Using FailingPayment instead of MockPayment
        let mut service = OrderService::new(MockRepository::new(), FailingPayment, MockSender);

        let items = vec![LineItem {
            name: "Test".to_string(),
            price: Money(1000),
        }];

        let result = service.place_order(items);

        assert!(matches!(result, Err(OrderError::PaymentFailed)));
    }

    #[test]
    fn get_order_returns_saved_order() {
        let mut service = OrderService::new(MockRepository::new(), MockPayment, MockSender);

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

// =============================================================================
// Key Takeaway
// =============================================================================
//
// The application crate is the GLUE between domain and the outside world.
// It knows about:
// - Domain entities (Order, Money, etc.)
// - Port traits (OrderRepository, PaymentGateway, Sender)
//
// It does NOT know about:
// - Concrete adapters (PostgreSQL, Stripe, SendGrid)
// - Infrastructure details (SQL, HTTP, etc.)
//
// This isolation means:
// - You can test without infrastructure
// - You can swap adapters without changing application code
// - The dependency graph is clean and predictable
//
// Next: check out the adapter crates to see concrete implementations!
