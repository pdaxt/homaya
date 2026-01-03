//! # Bernstein-Vazirani Algorithm
//!
//! Find a hidden n-bit string in ONE query.
//!
//! ## The Problem
//!
//! There's a secret string s = s₀s₁...s_(n-1).
//! Given a black-box function f(x) = s · x (dot product mod 2),
//! find s.
//!
//! Classically: Need n queries (ask about each bit).
//! Quantum: ONE query reveals s directly!
//!
//! ## How It Works
//!
//! The circuit is similar to Deutsch-Jozsa:
//! 1. Start with |0⟩^n |1⟩
//! 2. Apply H to all qubits
//! 3. Apply oracle Uf that computes f(x) = s · x
//! 4. Apply H to query qubits
//! 5. Measure query qubits → This IS the secret string s!
//!
//! ## The Magic
//!
//! Due to quantum interference, measuring the query qubits
//! directly gives you the secret string. No iterative guessing needed!

use homaya_core::Circuit;

/// Bernstein-Vazirani algorithm builder.
///
/// Creates a circuit that finds a hidden string in one query.
#[derive(Debug, Clone)]
pub struct BernsteinVazirani {
    /// Number of qubits (length of secret string)
    n_qubits: usize,
    /// The secret string we're trying to find
    secret: usize,
}

impl BernsteinVazirani {
    /// Create a new Bernstein-Vazirani instance.
    ///
    /// # Arguments
    ///
    /// * `n_qubits` - Number of bits in the secret string
    /// * `secret` - The secret string as an integer (0 to 2^n - 1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use homaya_algorithms::BernsteinVazirani;
    ///
    /// // Secret string is 101 (5 in decimal)
    /// let bv = BernsteinVazirani::new(3, 0b101);
    /// let circuit = bv.build();
    /// // Measuring will give "101" with 100% probability
    /// ```
    pub fn new(n_qubits: usize, secret: usize) -> Self {
        let max_secret = 1 << n_qubits;
        assert!(
            secret < max_secret,
            "Secret {} is too large for {} qubits (max: {})",
            secret,
            n_qubits,
            max_secret - 1
        );

        Self { n_qubits, secret }
    }

    /// Build the Bernstein-Vazirani circuit.
    ///
    /// After measurement, the query qubits will contain the secret string.
    pub fn build(&self) -> Circuit {
        let total_qubits = self.n_qubits + 1;
        let ancilla = self.n_qubits;

        let mut circuit = Circuit::new(total_qubits);

        // Step 1: Initialize ancilla to |1⟩
        circuit = circuit.x(ancilla);

        // Step 2: Apply H to all qubits
        for i in 0..total_qubits {
            circuit = circuit.h(i);
        }

        // Step 3: Apply oracle for f(x) = s · x
        // For each bit i where secret[i] = 1, apply CNOT from qubit i to ancilla
        for i in 0..self.n_qubits {
            if (self.secret >> i) & 1 == 1 {
                circuit = circuit.cx(i, ancilla);
            }
        }

        // Step 4: Apply H to query qubits
        for i in 0..self.n_qubits {
            circuit = circuit.h(i);
        }

        // Step 5: Measure query qubits
        for i in 0..self.n_qubits {
            circuit = circuit.measure(i, i);
        }

        circuit
    }

    /// Get the secret string as a binary string.
    ///
    /// Useful for verifying the measurement result.
    pub fn secret_as_binary(&self) -> String {
        format!("{:0width$b}", self.secret, width = self.n_qubits)
    }
}

/// Convenience function to create a Bernstein-Vazirani circuit.
///
/// # Example
///
/// ```rust
/// use homaya_algorithms::bernstein_vazirani;
///
/// let circuit = bernstein_vazirani::find_secret(4, 0b1010);
/// // Measuring gives "1010" with 100% probability
/// ```
pub fn find_secret(n_qubits: usize, secret: usize) -> Circuit {
    BernsteinVazirani::new(n_qubits, secret).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_size() {
        let bv = BernsteinVazirani::new(4, 0b1010);
        let circuit = bv.build();
        // 4 query qubits + 1 ancilla = 5 total
        assert_eq!(circuit.num_qubits(), 5);
    }

    #[test]
    fn test_secret_binary() {
        let bv = BernsteinVazirani::new(4, 0b1010);
        assert_eq!(bv.secret_as_binary(), "1010");

        let bv = BernsteinVazirani::new(4, 0b0101);
        assert_eq!(bv.secret_as_binary(), "0101");
    }

    #[test]
    #[should_panic(expected = "Secret 16 is too large")]
    fn test_invalid_secret() {
        BernsteinVazirani::new(4, 16);  // Max is 15 for 4 qubits
    }

    #[test]
    fn test_zero_secret() {
        let bv = BernsteinVazirani::new(3, 0);
        let circuit = bv.build();
        // Should still create valid circuit
        assert_eq!(circuit.num_qubits(), 4);
    }
}
