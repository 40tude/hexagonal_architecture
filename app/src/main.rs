// =============================================================================
// APP CRATE - The Composition Root
// =============================================================================
//
// Welcome to the "composition root" where everything comes together!
// This is the ONLY place in the entire workspace that knows about all crates.
//
// Look at Cargo.toml: we depend on EVERYTHING:
// - domain
// - application
// - adapters-repository
// - adapters-payment
// - adapters-notification
//
// This is intentional! The composition root is where you WIRE UP the application.
// It's where you decide: "For this run, use PostgreSQL, Stripe, and SendGrid."
// Or: "For tests, use InMemory, MockPayment, and ConsoleSender."
//
// IN DIP_06 VS HERE:
// ------------------
// In dip_06, main.rs was inside the same crate as everything else.
// Here, main.rs is in its own `app` crate that depends on all others.
//
// The benefit? MAXIMUM ISOLATION. Each crate compiles independently.
// Change the domain? Only domain and its dependents recompile.
// Change an adapter? Only that adapter crate recompiles.

use adapters_notification::{ConsoleSender, SendGridSender};
use adapters_payment::{MockPaymentGateway, StripePaymentGateway};
use adapters_repository::{InMemoryOrderRepository, PostgresOrderRepository};
use application::OrderService;
use domain::{LineItem, Money};

// =============================================================================
// Main Function - Same as dip_06!
// =============================================================================
//
// The beautiful part: despite all the crate separation, the USAGE CODE
// is identical to dip_06. We create adapters, inject them, use the service.
// The architecture change is invisible to the business logic!

fn main() {
    println!("=== Hexagonal Architecture Demo (Workspace) ===\n");

    // Test data: same as always
    let items = vec![
        LineItem {
            name: "Rust Programming Book".to_string(),
            price: Money(4999), // $49.99
        },
        LineItem {
            name: "Mechanical Keyboard".to_string(),
            price: Money(12999), // $129.99
        },
    ];

    // -------------------------------------------------------------------------
    // Configuration #1: In-Memory Adapters (Testing/Development)
    // -------------------------------------------------------------------------
    // Perfect for unit tests, local dev, CI pipelines.
    // No external services needed: everything runs in memory.
    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let sender = ConsoleSender;

        // Dependency Injection: we choose the adapters, service doesn't care!
        let mut service = OrderService::new(repo, payment, sender);

        match service.place_order(items.clone()) {
            Ok(order) => println!("\nOrder placed successfully: {}\n", order.id),
            Err(e) => println!("\nError: {e}\n"),
        }
    }

    // -------------------------------------------------------------------------
    // Configuration #2: External Services (Production)
    // -------------------------------------------------------------------------
    // Same OrderService, completely different adapters.
    // In a real app, you'd choose based on environment variables or config.
    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let sender = SendGridSender;

        // Same OrderService, production adapters!
        let mut service = OrderService::new(repo, payment, sender);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("\nOrder placed successfully: {}", order.id);

                // Demonstrate retrieval
                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "Retrieved order: {} items, total {}\n",
                        retrieved.items.len(),
                        retrieved.total
                    );
                }
            }
            Err(e) => println!("\nError: {e}\n"),
        }
    }
}

// =============================================================================
// What Have We Achieved?
// =============================================================================
//
// This workspace demonstrates hexagonal architecture at MAXIMUM SCALE:
//
// 1. DOMAIN CRATE: Pure business logic, zero dependencies
//    - Entities (Order, LineItem)
//    - Value Objects (OrderId, Money)
//    - Port Traits (OrderRepository, PaymentGateway, Sender)
//
// 2. APPLICATION CRATE: Use case orchestration
//    - Depends only on domain
//    - Contains OrderService
//    - Coordinates domain + ports
//
// 3. ADAPTER CRATES: Infrastructure implementations
//    - Each depends only on domain
//    - Isolated from each other
//    - Can have their own external dependencies
//
// 4. APP CRATE: Composition root
//    - Knows about everything
//    - Wires up the application
//    - Entry point for execution
//
// DEPENDENCY GRAPH:
//
//     app ───────────────────────────────────────┐
//      │                                         │
//      ├──► application ──► domain ◄─────────────┤
//      │                       ▲                 │
//      ├──► adapters-repository ┘                │
//      ├──► adapters-payment ────┘               │
//      └──► adapters-notification ───────────────┘
//
// All arrows point toward domain. That's DIP at the crate level!
//
// =============================================================================
// The Journey Complete
// =============================================================================
//
// dip_01: The problem: tight coupling
// dip_02: The solution: trait + dependency injection
// dip_03: Multiple adapters (Email, SMS, Owl)
// dip_04: Testing with mocks
// dip_05: Hexagonal architecture (single file)
// dip_06: Modular organization (folders)
// HERE:   Full workspace with independent crates
//
// You've seen DIP evolve from concept to production-ready architecture!
