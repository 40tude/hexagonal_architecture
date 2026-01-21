// =============================================================================
// ADAPTERS-NOTIFICATION CRATE - Sending Messages to Customers
// =============================================================================
//
// This crate provides concrete implementations of the `Sender` port.
// Remember dip_02? The Email struct was our first notification adapter.
// Here we have two: ConsoleSender (for testing) and SendGridSender (for production).
//
// WHY A SEPARATE CRATE?
// ---------------------
// In dip_06, all adapters lived in one `adapters/` folder.
// Here, each category of adapter has its own crate:
// - adapters-notification (this one)
// - adapters-payment
// - adapters-repository
//
// Benefits:
// 1. INDEPENDENT COMPILATION: Change notification code? Payment doesn't recompile.
// 2. SELECTIVE DEPENDENCIES: Only this crate needs sendgrid-rs (in production).
// 3. TEAM OWNERSHIP: The "notifications team" owns this crate.
// 4. CLEANER DEPENDENCY GRAPH: Each adapter category is isolated.
//
// DEPENDENCY DIRECTION:
// ---------------------
// Look at Cargo.toml: we depend on `domain`.
// Domain does NOT depend on us. The arrow points inward!
//
//     adapters-notification ───► domain
//
// This is DIP at the crate level.

mod console;
mod sendgrid;

// Re-export the public adapters.
// Users of this crate just write: `use adapters_notification::ConsoleSender;`
pub use console::ConsoleSender;
pub use sendgrid::SendGridSender;
