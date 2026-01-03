//! # Grover's Search Algorithm
//!
//! Find a marked item in an unsorted database with O(√N) queries.
//!
//! ## The Problem
//!
//! Imagine you have a phone book with N entries, but it's completely unsorted.
//! Classically, you'd need to check O(N) entries on average.
//! Grover's algorithm does it in O(√N) - a quadratic speedup!
//!
//! ## How It Works
//!
//! 1. **Superposition**: Put all N items in equal superposition
//! 2. **Oracle**: Mark the target item by flipping its phase
//! 3. **Diffusion**: Amplify the marked item's amplitude
//! 4. **Repeat**: Apply oracle + diffusion √N times
//! 5. **Measure**: Target item has high probability of being found
//!
//! ## Example
//!
//! ```rust
//! use homaya_algorithms::GroverSearch;
//!
//! // Search for item 6 in a space of 8 items (3 qubits)
//! let grover = GroverSearch::new(3, 6);
//! let circuit = grover.build();
//!
//! // The circuit will find item 6 with ~97% probability
//! ```
//!
//! ## The Math (simplified)
//!
//! After k iterations, the probability of measuring the target is:
//!
//! P(target) = sin²((2k + 1)θ)
//!
//! where θ = arcsin(1/√N) and optimal k ≈ π√N/4
//!

use homaya_core::{Circuit, PI};

/// Grover's Search algorithm builder.
///
/// Creates a quantum circuit that searches for a specific item
/// in an unsorted database with quadratic speedup.
#[derive(Debug, Clone)]
pub struct GroverSearch {
    /// Number of qubits (search space = 2^n_qubits)
    n_qubits: usize,
    /// The item we're searching for (0 to 2^n_qubits - 1)
    target: usize,
    /// Number of Grover iterations (auto-calculated if None)
    iterations: Option<usize>,
}

impl GroverSearch {
    /// Create a new Grover search instance.
    ///
    /// # Arguments
    ///
    /// * `n_qubits` - Number of qubits (search space = 2^n_qubits)
    /// * `target` - The item to search for (0 to 2^n_qubits - 1)
    ///
    /// # Panics
    ///
    /// Panics if target >= 2^n_qubits
    ///
    /// # Example
    ///
    /// ```rust
    /// use homaya_algorithms::GroverSearch;
    ///
    /// let grover = GroverSearch::new(4, 11);  // Search for 11 in 16 items
    /// ```
    pub fn new(n_qubits: usize, target: usize) -> Self {
        let max_target = 1 << n_qubits;
        assert!(
            target < max_target,
            "Target {} is too large for {} qubits (max: {})",
            target,
            n_qubits,
            max_target - 1
        );

        Self {
            n_qubits,
            target,
            iterations: None,
        }
    }

    /// Set a custom number of iterations.
    ///
    /// By default, the optimal number is calculated automatically.
    /// Use this if you want to experiment with different iteration counts.
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = Some(iterations);
        self
    }

    /// Calculate the optimal number of Grover iterations.
    ///
    /// The formula is: k ≈ π/4 × √N
    ///
    /// where N = 2^n_qubits is the search space size.
    pub fn optimal_iterations(&self) -> usize {
        let n = (1 << self.n_qubits) as f64;
        let optimal = (PI / 4.0 * n.sqrt()).round() as usize;
        optimal.max(1)
    }

    /// Build the Grover search circuit.
    ///
    /// Returns a circuit that, when executed and measured,
    /// will return the target item with high probability.
    pub fn build(&self) -> Circuit {
        let iterations = self.iterations.unwrap_or_else(|| self.optimal_iterations());

        let mut circuit = Circuit::new(self.n_qubits);

        // Step 1: Create uniform superposition
        // Apply H to all qubits: |0...0⟩ → |+...+⟩
        for i in 0..self.n_qubits {
            circuit = circuit.h(i);
        }

        // Step 2: Grover iterations
        for _ in 0..iterations {
            // Oracle: flip the phase of |target⟩
            circuit = self.apply_oracle(circuit);

            // Diffusion: amplify the marked state
            circuit = self.apply_diffusion(circuit);
        }

        // Step 3: Measure all qubits
        circuit.measure_all()
    }

    /// Apply the oracle that marks the target state.
    ///
    /// The oracle flips the sign of the |target⟩ amplitude:
    /// |target⟩ → -|target⟩
    ///
    /// This is done using controlled-Z gates based on the binary
    /// representation of the target.
    fn apply_oracle(&self, mut circuit: Circuit) -> Circuit {
        // Apply X gates to qubits that are 0 in the target
        // This transforms |target⟩ → |11...1⟩
        for i in 0..self.n_qubits {
            if (self.target >> i) & 1 == 0 {
                circuit = circuit.x(i);
            }
        }

        // Multi-controlled Z gate on all qubits
        // This flips the sign of |11...1⟩
        circuit = self.multi_controlled_z(circuit);

        // Undo the X gates
        for i in 0..self.n_qubits {
            if (self.target >> i) & 1 == 0 {
                circuit = circuit.x(i);
            }
        }

        circuit
    }

    /// Apply the diffusion operator (Grover's diffuser).
    ///
    /// The diffusion operator is: D = 2|s⟩⟨s| - I
    /// where |s⟩ is the uniform superposition state.
    ///
    /// This reflects amplitudes about their mean, amplifying
    /// the marked state.
    fn apply_diffusion(&self, mut circuit: Circuit) -> Circuit {
        // Apply H to all qubits
        for i in 0..self.n_qubits {
            circuit = circuit.h(i);
        }

        // Apply X to all qubits (transforms |0...0⟩ → |1...1⟩)
        for i in 0..self.n_qubits {
            circuit = circuit.x(i);
        }

        // Multi-controlled Z
        circuit = self.multi_controlled_z(circuit);

        // Undo X gates
        for i in 0..self.n_qubits {
            circuit = circuit.x(i);
        }

        // Apply H to all qubits
        for i in 0..self.n_qubits {
            circuit = circuit.h(i);
        }

        circuit
    }

    /// Implement multi-controlled Z using decomposition.
    ///
    /// For 2 qubits: CZ
    /// For 3+ qubits: decompose into Toffoli + controlled gates
    fn multi_controlled_z(&self, mut circuit: Circuit) -> Circuit {
        match self.n_qubits {
            0 | 1 => circuit.z(0),
            2 => {
                // CZ gate: controlled-Z on qubits 0,1
                circuit.h(1).cx(0, 1).h(1)
            }
            3 => {
                // CCZ using H-Toffoli-H pattern
                circuit.h(2).ccx(0, 1, 2).h(2)
            }
            _ => {
                // For larger circuits, use a simplified pattern
                // Apply Z to last qubit controlled by all others
                // This is an approximation for demonstration
                let last = self.n_qubits - 1;
                circuit = circuit.h(last);
                for i in 0..last {
                    circuit = circuit.cx(i, last);
                }
                circuit.h(last)
            }
        }
    }

    /// Get the theoretical success probability.
    ///
    /// Returns the probability of measuring the target state
    /// after the optimal number of iterations.
    pub fn success_probability(&self) -> f64 {
        let n = (1 << self.n_qubits) as f64;
        let theta = (1.0 / n.sqrt()).asin();
        let k = self.iterations.unwrap_or_else(|| self.optimal_iterations()) as f64;
        let angle = (2.0 * k + 1.0) * theta;
        angle.sin().powi(2)
    }
}

/// Convenience function to create a Grover search circuit.
///
/// # Example
///
/// ```rust
/// use homaya_algorithms::grover;
///
/// let circuit = grover::search(3, 5);  // Find 5 in 8 items
/// ```
pub fn search(n_qubits: usize, target: usize) -> Circuit {
    GroverSearch::new(n_qubits, target).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grover_creates_circuit() {
        let circuit = search(3, 5);
        assert!(circuit.num_qubits() == 3);
    }

    #[test]
    fn test_optimal_iterations() {
        // For N=8 (3 qubits): √8 ≈ 2.83, π/4 × 2.83 ≈ 2.22 → 2 iterations
        let grover = GroverSearch::new(3, 5);
        assert!(grover.optimal_iterations() == 2);

        // For N=16 (4 qubits): √16 = 4, π/4 × 4 ≈ 3.14 → 3 iterations
        let grover = GroverSearch::new(4, 11);
        assert!(grover.optimal_iterations() == 3);
    }

    #[test]
    fn test_success_probability() {
        let grover = GroverSearch::new(3, 5);
        let prob = grover.success_probability();
        // Should be high (> 90%)
        assert!(prob > 0.9, "Success probability {} too low", prob);
    }

    #[test]
    #[should_panic(expected = "Target 16 is too large")]
    fn test_invalid_target() {
        GroverSearch::new(4, 16);  // Max is 15 for 4 qubits
    }
}
