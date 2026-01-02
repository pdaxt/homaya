<p align="center">
  <h1 align="center">🌀 QUASAR</h1>
</p>

<p align="center">
  <strong>Quantum Unified Architecture for Simulation And Runtime</strong>
</p>

<p align="center">
  <em>Learning quantum computing by building it from scratch</em>
</p>

<p align="center">
  <a href="#what-is-this">What is This?</a> •
  <a href="#learn-with-us">Learn With Us</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#project-status">Status</a> •
  <a href="#contributing">Contributing</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/status-learning%20project-purple.svg" alt="Status">
</p>

---

## What is This?

**We're learning quantum computing by building a quantum computing framework from scratch.**

This isn't a polished production library (yet). It's a learning journey, documented in code. We believe the best way to truly understand something is to build it yourself.

### Our Philosophy

```
🎯 Learn by building, not just reading
📖 Document everything we discover
🤝 Make it accessible to complete beginners
🔬 Verify our understanding with real tests
```

### What We're Building

| Component | Purpose | Status |
|-----------|---------|--------|
| **quasar-core** | Qubits, gates, circuits - the fundamentals | ✅ Working |
| **quasar-sim** | Simulate quantum circuits on your computer | ✅ Working |
| **Interactive Course** | Learn quantum computing step-by-step | ✅ Available |
| **Verification Tests** | Prove our simulator follows real physics | ✅ Passing |

---

## 🎓 Learn With Us

### Interactive Course

We built an interactive course that explains quantum computing like you're 10 years old:

**[→ Open the Interactive Course](docs/learn/course.html)**

The course covers:
- 💡 **Light switches & bits** - What computers actually do
- 🪙 **The spinning coin** - What makes quantum different
- ⚖️ **Weighted coins** - Controlling probabilities
- 🔘 **The H button** - The most important quantum operation
- 🔧 **Building circuits** - Putting it all together

No math prerequisites. No physics degree needed. Just curiosity!

### Why We Made This

We were frustrated. Every quantum computing resource either:
- Assumed you had a physics PhD, or
- Was so oversimplified it didn't actually teach anything

We wanted something in between. So we built it.

---

## Quick Start

### Run the Simulator

```bash
# Clone the repo
git clone https://github.com/anthropics/quasar.git
cd quasar

# Run the demo
cargo run --example sim_demo -p quasar-sim

# Verify correctness (proves it follows quantum mechanics!)
cargo run --example verify_correctness -p quasar-sim
```

### Build Your First Circuit

```rust
use quasar_core::Circuit;
use quasar_sim::Simulator;

// Create a "Bell state" - two qubits that are connected
let circuit = Circuit::new(2)
    .h(0)        // Make qubit 0 "spin" (superposition)
    .cx(0, 1)    // Connect qubit 1 to qubit 0
    .measure_all();

// Simulate it!
let mut sim = Simulator::new();
let counts = sim.sample(&circuit, 1000).unwrap();

// You'll get roughly 50% "00" and 50% "11"
// Never "01" or "10" - they're entangled!
println!("{:?}", counts);
```

### Open the Course

```bash
# Just open this file in your browser:
open docs/learn/course.html
```

---

## Project Status

### What Works ✅

- **Core primitives**: Qubits, complex numbers, all standard gates
- **Simulator**: Full state vector simulation with measurement
- **Gates**: X, Y, Z, H, S, T, Rx, Ry, Rz, CNOT, CZ, SWAP, Toffoli
- **Verification**: 8 mathematical tests proving correctness
- **Interactive course**: Beginner-friendly explanations

### What We're Building 🚧

- [ ] More interactive examples in the course
- [ ] Two-qubit entanglement lesson
- [ ] Python bindings
- [ ] Browser/WASM support
- [ ] GPU acceleration

### What We Verified

Our simulator passes these physics tests:

| Test | What It Proves |
|------|---------------|
| Probability Conservation | All probabilities sum to 1 (Born rule) |
| H² = I | Hadamard is self-inverse |
| X² = I | Pauli-X is self-inverse |
| Rx(2π) = I | Full rotation returns to start |
| Bell State | Creates perfect |00⟩ + |11⟩ entanglement |
| CNOT Truth Table | Controlled-NOT works correctly |
| Statistical Sampling | Measurements follow expected probabilities |
| Hadamard Superposition | Creates exact 50/50 distribution |

---

## Architecture

```
quasar/
├── crates/
│   ├── quasar-core/      # Qubits, gates, circuits
│   ├── quasar-sim/       # State vector simulator
│   ├── quasar-ir/        # Intermediate representation (planned)
│   ├── quasar-compiler/  # Optimization (planned)
│   ├── quasar-runtime/   # Execution (planned)
│   ├── quasar-backends/  # Hardware backends (planned)
│   └── quasar-algorithms/# Standard algorithms (planned)
├── docs/
│   └── learn/            # Interactive course
├── examples/
│   └── rust/             # Example programs
└── tools/
    └── quasar-cli/       # Command line tool (planned)
```

---

## Contributing

**We'd love to learn with you!**

This project is explicitly for learning. You don't need to be an expert. In fact, questions from beginners help us write better explanations.

### Ways to Contribute

1. **Try the course** and tell us what's confusing
2. **Ask questions** - if you're confused, others are too
3. **Fix explanations** that don't make sense
4. **Add more examples** to the course
5. **Improve the simulator** with new features

### Running Tests

```bash
# Run all tests
cargo test

# Run the verification suite
cargo run --example verify_correctness -p quasar-sim
```

---

## Resources We Used

Learning quantum computing is hard. Here are resources that helped us:

- [Quantum Computing: An Applied Approach](https://link.springer.com/book/10.1007/978-3-030-23922-0) - Jack Hidary
- [Qiskit Textbook](https://qiskit.org/textbook/) - IBM
- [Quantum Country](https://quantum.country/) - Andy Matuschak & Michael Nielsen
- [3Blue1Brown](https://www.youtube.com/c/3blue1brown) - Visual math intuition

---

## License

MIT / Apache 2.0 - Use it however you want.

---

<p align="center">
  <strong>Learning in public. Building in the open.</strong>
</p>

<p align="center">
  <em>If you're confused, that's okay. So were we. That's why we built this.</em>
</p>
