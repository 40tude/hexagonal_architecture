# Hexagonal Architecture Demo


A Rust workspace demonstrating Hexagonal Architecture (Ports & Adapters) with Dependency Inversion Principle (DIP).



> **Warning (Linux/macOS users):** The .cargo/ folder contains Windows-specific configuration. Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


* Read this page if you look for a [Beginner’s Guide to Hexagonal Architecture](https://www.40tude.fr/docs/06_programmation/rust/024_hexagonal/hexagonal_lite.html) in Rust
* You can learn more about SOLID and DIP with [Episode 04](https://www.40tude.fr/docs/06_programmation/rust/022_solid/solid_04.html) of this set of posts about SOLID.




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
037_solid_hexagonal_architecture/
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
│       ├── console.rs          # ConsoleSender
│       └── sendgrid.rs         # SendGridSender (simulated)
└── app/                        # Application entry point
    └── src/main.rs             # Demo with swappable adapters
```

## Dependency Inversion Principle

The key insight is that **dependencies point inward**:

1. **domain**: Zero dependencies. Defines business entities and port traits.
2. **application**: Depends only on domain. Contains use cases.
3. **adapters-***: Repository, Payment and Notification. Each depends only on domain. Implements port traits.
4. **app**: Composes everything. Wires adapters to application services.

Adapters implement domain-defined traits (ports), not the other way around. This means:
- Domain never knows about infrastructure
- Adapters can be swapped without changing business logic
- Testing is simplified via mock adapters

## Port Traits (defined in domain)

```rust
pub trait OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), OrderError>;
    fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
}

pub trait PaymentGateway {
    fn charge(&self, amount: Money) -> Result<(), OrderError>;
}

pub trait Sender {
    fn send(&self, order: &Order) -> Result<(), OrderError>;
}
```

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
=== Hexagonal Architecture Demo (Workspace) ===

--- Configuration #1: In-Memory Adapters (Testing) ---

  [Mock] Charging $179.98
  [InMemory] Saving order #OrderId(1)
  [Console] Order #OrderId(1) confirmed! Total: $179.98

Order placed successfully: OrderId(1)

--- Configuration #2: External Services (Production) ---

  [Stripe API] POST /charges amount=$179.98
  [Postgres] INSERT INTO orders VALUES (OrderId(1), ...)
  [SendGrid API] Sending email: 'Order #OrderId(1) Confirmed'

Order placed successfully: OrderId(1)

  [Postgres] SELECT * FROM orders WHERE id = OrderId(1)
Retrieved order: 2 items, total $179.98
```

## Related Examples

Again, this workspace is part of a series demonstrating DIP evolution. Read [Episode 04](https://www.40tude.fr/docs/06_programmation/rust/022_solid/solid_01.html) of this set of posts about SOLID.

| Example | Description |
|---------|-------------|
| dip_01 | The problem: tight coupling |
| dip_02 | The solution: trait + dependency injection |
| dip_03 | Multiple adapters (Email, SMS, Owl) |
| dip_04 | Testing with mocks |
| dip_05 | Hexagonal architecture (single file) |
| dip_06 | Modular organization (folders) |
| **here** | Full workspace with independent crates |


## Is This a Modular Monolith?

Yes because the project demonstrates a "textbook" Hexagonal Architecture with (I hope) a clean module separation. It's modular (independent crates with enforced boundaries) and monolithic (single deployable binary). The architectural patterns shown are production-ready and scalable.

More explanation below:

### 1. Single Deployment Unit (Monolith criterion Ok)
- All 6 crates compile into one binary via `cargo run -p app`
- No microservice separation, no network boundaries between components

### 2. Module Boundaries (Modular criterion Ok)
- 6 independent crates with explicit Cargo dependencies
- Strict dependency inversion: all adapters depend only on domain, never on each other
- Dependency direction enforced by compiler: `app → application → domain ← adapters-*`

### 3. Domain Isolation (Ok)
- `domain` crate has zero external dependencies
- Port traits (`OrderRepository`, `PaymentGateway`, `Sender`) define contracts
- Adapters implement ports without domain knowing about infrastructure

### 4. Swappable Components (Ok)
- `app/src/main.rs` demonstrates two configurations:
- Testing: `InMemoryOrderRepository` + `MockPaymentGateway` + `ConsoleSender`
- Production: `PostgresOrderRepository` + `StripePaymentGateway` + `SendGridSender`

### Caveats
This is a demonstration-scale modular monolith. Production systems would typically include:
- Multiple bounded contexts (not just Order)
- Async traits for I/O operations
- Inter-domain communication mechanisms
- Shared infrastructure crate (logging, config)




## License

MIT License - see [LICENSE](LICENSE) for details.
