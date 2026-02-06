# Architecture Decision Records

## ADR-001: Rust as Implementation Language

**Status:** Accepted  
**Date:** 2025-02-04

### Context
Building computational lexicography for Yiddish requires performance, memory safety, and strong typing.

### Decision
Use Rust for all infrastructure and business logic.

### Consequences
- **Positive:** Memory safety, zero-cost abstractions, compile-time guarantees
- **Negative:** Steeper learning curve, longer initial development
- **Neutral:** Requires PostgreSQL, binary distribution

---

## ADR-002: Type-Safe Error Handling

**Status:** Accepted  
**Date:** 2025-02-04

### Decision
Use thiserror + miette for structured, pretty error printing.

---

## ADR-003: Property-Based Testing

**Status:** Accepted  
**Date:** 2025-02-04

### Decision
Use proptest to verify morphological invariants across all inputs.
