//! This library exposes foo for fooing:
#![cfg_attr(not(feature = "foo"), doc = "```ignore")]
#![cfg_attr(feature = "foo", doc = "```rust")]
//! use doctest_cfg_line_numbers::foo;
//! foo();
//! ```
//!
//! But baring is also possible:
#![cfg_attr(not(feature = "bar"), doc = "```ignore")]
#![cfg_attr(feature = "bar", doc = "```rust")]
//! use doctest_cfg_line_numbers::bar;
//! bar();
//! ```
//!
//! The above doctests would fail without the guards.
//! Another approach is:
//! ```rust
//! # #[cfg(feature = "foo")]
//! # {
//! use doctest_cfg_line_numbers::foo;
//! foo()
//! # }
//!
//! # #[cfg(feature = "bar")]
//! # {
//! use doctest_cfg_line_numbers::bar;
//! bar()
//! # }
//! ```
//!
//! The issue is that the above test with the inline workaround will always pass even though it didn't really run!!!!
//! The other tests are reported as ignored when they are ignored as the should be.
//!
//! When we run `cargo test --features bar` we get:
//! ```text
//! failures:
//!   src/lib.rs - (line 15)
//!   src/lib.rs - (line 8)
//! ```
//!
//! The line numbers are incorrect and it is inconvenient to go there from the log output.
//! Rust usually uses the notation `thread 'main' (2413146) panicked at src/lib.rs:36:5:` which is well understood by editors.
//! The failure output should ideally also use this format not `- (line 15)`.

#[cfg(feature = "bar")]
pub fn bar() {
    assert!(false, "Bar always fails")
}
#[cfg(feature = "foo")]
pub fn foo() {}
