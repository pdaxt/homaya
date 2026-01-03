//! # HOMAYA Algorithms
//!
//! Quantum algorithms that demonstrate the power of quantum computing.
//!
//! ## Available Algorithms
//!
//! - [`grover`] - Grover's Search: Find a needle in a haystack with √N queries
//! - [`deutsch`] - Deutsch-Jozsa: Determine if a function is constant or balanced
//! - [`bernstein_vazirani`] - Find a hidden string in one query
//!
//! ## Example: Grover's Search
//!
//! ```rust
//! use homaya_algorithms::grover;
//!
//! // Search for item 5 in a database of 8 items
//! let circuit = grover::search(3, 5);  // 3 qubits = 8 items
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod grover;
pub mod deutsch;
pub mod bernstein_vazirani;

pub use grover::GroverSearch;
pub use deutsch::DeutschJozsa;
pub use bernstein_vazirani::BernsteinVazirani;
