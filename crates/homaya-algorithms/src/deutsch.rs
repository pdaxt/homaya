//! # Deutsch-Jozsa Algorithm
//!
//! Determine if a function is constant or balanced in ONE query.
//!
//! ## The Problem
//!
//! Given a black-box function f: {0,1}ⁿ → {0,1} that is promised to be either:
//! - **Constant**: f(x) = 0 for all x, OR f(x) = 1 for all x
//! - **Balanced**: f(x) = 0 for exactly half of inputs, f(x) = 1 for the other half
//!
//! Classically, you'd need up to 2^(n-1) + 1 queries in the worst case.
//! Deutsch-Jozsa solves it with just ONE quantum query!
//!
//! ## How It Works
//!
//! 1. Start with |0⟩^n |1⟩ (n query qubits + 1 ancilla)
//! 2. Apply H to all qubits
//! 3. Apply the oracle Uf
//! 4. Apply H to query qubits
//! 5. Measure query qubits:
//!    - All zeros → Constant
//!    - Any non-zero → Balanced
//!

use homaya_core::Circuit;

/// Types of functions for Deutsch-Jozsa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    /// f(x) = 0 for all x
    ConstantZero,
    /// f(x) = 1 for all x
    ConstantOne,
    /// f(x) = parity of x (balanced)
    BalancedParity,
    /// f(x) = first bit of x (balanced)
    BalancedFirstBit,
}

/// Deutsch-Jozsa algorithm builder.
///
/// Creates a circuit that determines if a function is constant or balanced.
#[derive(Debug, Clone)]
pub struct DeutschJozsa {
    /// Number of query qubits
    n_qubits: usize,
    /// The oracle function type
    function: FunctionType,
}

impl DeutschJozsa {
    /// Create a new Deutsch-Jozsa instance.
    ///
    /// # Arguments
    ///
    /// * `n_qubits` - Number of query qubits (domain is {0,1}^n)
    /// * `function` - The oracle function type
    ///
    /// # Example
    ///
    /// ```rust
    /// use homaya_algorithms::{DeutschJozsa, deutsch::FunctionType};
    ///
    /// let dj = DeutschJozsa::new(3, FunctionType::BalancedParity);
    /// ```
    pub fn new(n_qubits: usize, function: FunctionType) -> Self {
        assert!(n_qubits >= 1, "Need at least 1 query qubit");
        Self { n_qubits, function }
    }

    /// Build the Deutsch-Jozsa circuit.
    ///
    /// The circuit has n+1 qubits: n query qubits + 1 ancilla.
    /// After measurement, check if query qubits are all zero.
    pub fn build(&self) -> Circuit {
        let total_qubits = self.n_qubits + 1;
        let ancilla = self.n_qubits;  // Last qubit is ancilla

        let mut circuit = Circuit::new(total_qubits);

        // Step 1: Initialize ancilla to |1⟩
        circuit = circuit.x(ancilla);

        // Step 2: Apply H to all qubits
        for i in 0..total_qubits {
            circuit = circuit.h(i);
        }

        // Step 3: Apply the oracle
        circuit = self.apply_oracle(circuit, ancilla);

        // Step 4: Apply H to query qubits (not ancilla)
        for i in 0..self.n_qubits {
            circuit = circuit.h(i);
        }

        // Step 5: Measure query qubits
        for i in 0..self.n_qubits {
            circuit = circuit.measure(i, i);
        }

        circuit
    }

    /// Apply the oracle based on function type.
    fn apply_oracle(&self, mut circuit: Circuit, ancilla: usize) -> Circuit {
        match self.function {
            FunctionType::ConstantZero => {
                // f(x) = 0: do nothing (identity)
                circuit
            }
            FunctionType::ConstantOne => {
                // f(x) = 1: flip ancilla unconditionally
                circuit.x(ancilla)
            }
            FunctionType::BalancedParity => {
                // f(x) = x_0 XOR x_1 XOR ... XOR x_(n-1)
                // Apply CNOT from each query qubit to ancilla
                for i in 0..self.n_qubits {
                    circuit = circuit.cx(i, ancilla);
                }
                circuit
            }
            FunctionType::BalancedFirstBit => {
                // f(x) = x_0 (first bit)
                circuit.cx(0, ancilla)
            }
        }
    }

    /// Check if the function is constant based on measurement result.
    ///
    /// Returns true if the measurement string indicates a constant function.
    pub fn is_constant(measurement: &str) -> bool {
        measurement.chars().all(|c| c == '0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_size() {
        let dj = DeutschJozsa::new(3, FunctionType::BalancedParity);
        let circuit = dj.build();
        // 3 query qubits + 1 ancilla = 4 total
        assert_eq!(circuit.num_qubits(), 4);
    }

    #[test]
    fn test_all_function_types() {
        for func in [
            FunctionType::ConstantZero,
            FunctionType::ConstantOne,
            FunctionType::BalancedParity,
            FunctionType::BalancedFirstBit,
        ] {
            let dj = DeutschJozsa::new(2, func);
            let circuit = dj.build();
            assert_eq!(circuit.num_qubits(), 3);
        }
    }

    #[test]
    fn test_is_constant() {
        assert!(DeutschJozsa::is_constant("000"));
        assert!(DeutschJozsa::is_constant("0000"));
        assert!(!DeutschJozsa::is_constant("001"));
        assert!(!DeutschJozsa::is_constant("100"));
    }
}
