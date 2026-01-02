# Contributing to QUASAR

> *"We're not just writing code. We're building the infrastructure for the quantum era."*

Thank you for your interest in contributing to QUASAR. This document outlines our contribution process, standards, and the AI-powered review system that ensures every contribution meets our quality bar.

---

## The QUASAR Standard

### Core Principles

1. **Every line has purpose** - No dead code, no "just in case" additions
2. **Speed is non-negotiable** - Profile before and after. No regressions.
3. **Simplicity over cleverness** - Code is read more than written
4. **Documentation is mandatory** - If it's not documented, it doesn't exist
5. **Tests prove correctness** - No merge without tests

---

## AI-Powered Code Review

Every pull request goes through our AI review system before human review. This isn't optional.

### What the AI Checks

| Check | Description | Must Pass |
|-------|-------------|-----------|
| **Quality** | Code style, complexity, patterns | ✅ |
| **Performance** | Allocation analysis, hot paths | ✅ |
| **Security** | Unsafe code, input validation | ✅ |
| **Documentation** | Public API docs, examples | ✅ |
| **Tests** | Coverage, edge cases | ✅ |
| **Breaking Changes** | API compatibility | ⚠️ Review |

### How It Works

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Your PR   │────▶│   AI Agent   │────▶│   Review    │
│             │     │              │     │   Report    │
└─────────────┘     └──────────────┘     └──────┬──────┘
                                                │
                           ┌────────────────────┴────────────────────┐
                           │                                         │
                           ▼                                         ▼
                    ┌─────────────┐                           ┌─────────────┐
                    │    PASS     │                           │    FAIL     │
                    │             │                           │             │
                    │ Human Review│                           │ Fix Issues  │
                    └─────────────┘                           └─────────────┘
```

### AI Review Commands

In your PR, you can interact with the AI reviewer:

```
@quasar-bot review          # Trigger a review
@quasar-bot explain <line>  # Explain a suggestion
@quasar-bot benchmark       # Run performance comparison
@quasar-bot approve         # (Maintainers only) Override
```

---

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Python 3.10+ (for Python bindings)
- Node.js 18+ (for JavaScript bindings)
- wasm-pack (for WebAssembly)

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/quasar-quantum/quasar.git
cd quasar

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench
```

### IDE Setup

We recommend VS Code with:
- `rust-analyzer` extension
- `Even Better TOML` extension
- `CodeLLDB` for debugging

---

## Code Standards

### Rust Style

```rust
// ✅ Good: Clear, documented, tested
/// Applies a Hadamard gate to create superposition.
///
/// # Example
/// ```
/// let circuit = Circuit::new(1).h(0);
/// ```
#[inline]
pub fn h(mut self, qubit: usize) -> Self {
    self.push(Instruction::new(Gate::h(), vec![qubit]));
    self
}

// ❌ Bad: Unclear, undocumented
pub fn h(mut self, q: usize) -> Self {
    self.instructions.push(Instruction::new(Gate::h(), vec![q]));
    self
}
```

### Performance Rules

1. **No allocations in hot paths** - Use stack or pre-allocated buffers
2. **Prefer iteration over indexing** - `for x in &vec` not `for i in 0..vec.len()`
3. **Use `#[inline]` judiciously** - Only for small, frequently-called functions
4. **Profile before optimizing** - Don't guess, measure

### Documentation Rules

1. **Every public item must have a doc comment**
2. **Include examples in doc comments**
3. **Explain the "why", not just the "what"**
4. **Use `# Panics`, `# Errors`, `# Safety` sections**

---

## Pull Request Process

### Before Submitting

- [ ] Run `cargo fmt --all`
- [ ] Run `cargo clippy --workspace -- -D warnings`
- [ ] Run `cargo test --workspace`
- [ ] Run `cargo doc --workspace --no-deps`
- [ ] Update CHANGELOG.md if applicable

### PR Template

```markdown
## Description
[What does this PR do?]

## Motivation
[Why is this change needed?]

## Changes
- [Change 1]
- [Change 2]

## Performance Impact
[Did you run benchmarks? Include results.]

## Testing
[How did you test this?]

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] No performance regression
```

### Review Process

1. **AI Review** - Automated checks run first
2. **Human Review** - Maintainer reviews code
3. **Iteration** - Address feedback
4. **Approval** - Two approvals required for merge
5. **Merge** - Squash and merge to main

---

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add GPU acceleration for state vector simulation
fix: correct phase in Rz gate implementation
perf: optimize Hadamard gate application with SIMD
docs: add example for GHZ state preparation
test: add property tests for circuit composition
refactor: extract gate matrix computation
chore: update dependencies
```

---

## Issue Reporting

### Bug Reports

Use the bug report template:
- Clear title
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Minimal reproduction code

### Feature Requests

- Describe the use case
- Explain why existing features don't suffice
- Propose a solution (optional)

---

## Security

If you discover a security vulnerability:

1. **DO NOT** open a public issue
2. Email security@quasar.quantum
3. Include detailed reproduction steps
4. Allow 90 days for fix before disclosure

---

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md
- Release notes
- Our website's contributor page

Top contributors may be invited to join the core team.

---

## Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/). In short:

- Be respectful
- Be constructive
- Be inclusive

Violations are handled by the core team.

---

## Questions?

- Open a [Discussion](https://github.com/quasar-quantum/quasar/discussions)
- Join our [Discord](https://discord.gg/quasar-quantum)
- Email contributors@quasar.quantum

---

*"The best code is code that doesn't need to exist. The second best is code that's so clear it explains itself."*
