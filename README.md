# Hexagonal Architecture Demo
Readd this [Episode 04](https://www.40tude.fr/docs/06_programmation/rust/022_solid/solid_01.html) of this set of posts about SOLID.

A Rust workspace demonstrating Hexagonal Architecture (Ports & Adapters) with Dependency Inversion Principle (DIP).

> **Warning (Linux/macOS users):** The .cargo/ folder contains Windows-specific configuration. Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```

## Architecture Overview

```
                    ┌──────────────────────┐
                    │        app           │
                    └──────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────────┐
│adapters-repo  │  │adapters-pay   │  │adapters-notif     │
└───────────────┘  └───────────────┘  └───────────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              ▼
                    ┌──────────────────────┐
                    │     application      │
                    └──────────────────────┘
                              │
                              ▼
                    ┌──────────────────────┐
                    │       domain         │
                    └──────────────────────┘
```

## Workspace Structure

```
043_hexagonal_architecture/
├── domain/                     # Core business logic (no dependencies)
│   └── src/lib.rs              # OrderId, Money, Order, port traits
├── application/                # Use cases (depends on domain)
│   └── src/lib.rs              # OrderService<R,P,N>
├── adapters-repository/        # Repository implementations
│   └── src/
│       ├── in_memory.rs        # InMemoryOrderRepository
│       └── postgres.rs         # PostgresOrderRepository (simulated)
├── adapters-payment/           # Payment implementations
│   └── src/
│       ├── mock.rs             # MockPaymentGateway
│       └── stripe.rs           # StripePaymentGateway (simulated)
├── adapters-notification/      # Notification implementations
│   └── src/
│       ├── console.rs          # ConsoleNotificationService
│       └── sendgrid.rs         # SendGridNotificationService (simulated)
└── app/                        # Application entry point
    └── src/main.rs             # Demo with swappable adapters
```

## Dependency Inversion Principle

The key insight is that **dependencies point inward**:

1. **domain**: Zero dependencies. Defines business entities and port traits.
2. **application**: Depends only on domain. Contains use cases.
3. **adapters-***: Each depends only on domain. Implements port traits.
4. **app**: Composes everything. Wires adapters to application services.

Adapters implement domain-defined traits (ports), not the other way around. This means:
- Domain never knows about infrastructure
- Adapters can be swapped without changing business logic
- Testing is simplified via mock adapters

## Usage

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run demo
cargo run -p app
```

## Expected Output

```
=== Hexagonal Architecture Demo ===

--- Configuration #1: In-Memory Adapters (Testing) ---

[Mock] Charging $179.98
[InMemory] Saving order #OrderId(1)
[Console] Order #OrderId(1) confirmed - Total: $179.98
Order placed successfully: OrderId(1)

--- Configuration #2: External Services (Production) ---

[Stripe API] POST /charges amount=$179.98
[Postgres] INSERT INTO orders VALUES (OrderId(1), ...)
[SendGrid API] POST /mail/send to=customer@example.com subject='Order #OrderId(1) Confirmed'
Order placed successfully: OrderId(1)

[Postgres] SELECT * FROM orders WHERE id = OrderId(1)
Retrieved order: 2 items, total $179.98
```

## License

MIT License - see [LICENSE](LICENSE) for details.
