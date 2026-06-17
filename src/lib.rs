//! AnchorEdit v2 — Lightweight apply engine built on AnchorScope.
//!
//! # Usage
//!
//! ```no_run
//! use anchoredit::apply;
//!
//! let result = apply("src/main.rs", b"fn main() {}", b"fn main() { println!(\"hello\"); }")?;
//! println!("wrote {} bytes, scope_hash={}", result.bytes_written, result.scope_hash);
//! # Ok::<(), anchoredit::ApplyError>(())
//! ```

pub mod apply;
pub mod error;

pub use apply::{apply, ApplyResult, ApplyError};
