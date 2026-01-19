// Rust guideline compliant 2025-05-15
//! Hexagonal Architecture Demo Application.
//!
//! Demonstrates dependency injection with swappable adapters.

use adapters_notification::{ConsoleNotificationService, SendGridNotificationService};
use adapters_payment::{MockPaymentGateway, StripePaymentGateway};
use adapters_repository::{InMemoryOrderRepository, PostgresOrderRepository};
use application::OrderService;
use domain::{LineItem, Money};

fn main() {
    println!("=== Hexagonal Architecture Demo ===\n");

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

    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let notifications = ConsoleNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => println!("Order placed successfully: {:?}\n", order.id),
            Err(e) => println!("Error: {e}\n"),
        }
    }

    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let notifications = SendGridNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("Order placed successfully: {:?}", order.id);

                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "Retrieved order: {} items, total {}",
                        retrieved.items.len(),
                        retrieved.total
                    );
                }
            }
            Err(e) => println!("Error: {e}"),
        }
    }
}
