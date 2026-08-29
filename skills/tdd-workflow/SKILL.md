---
name: tdd-workflow
description: Test-Driven Development protocol enforcing test verification and incremental implementation before code generation.
version: 1.0.0
triggers:
  - "tdd workflow"
  - "write tests first"
  - "incremental test"
  - "unit test strategy"
execution_mode: auto
default_turns: 2
---

# 🧪 Test-Driven Development (TDD) Protocol

This skill enforces strict, incremental software engineering through test-first implementation patterns.

## 📋 TDD Execution Rules
1. **Define Test Scenarios First:** Before writing production implementations, write unit/integration tests establishing expected inputs and exact outputs (e.g. `assert_eq!(result, expected)`).
2. **Avoid Trivial Assertions:** Never use superficial assertions like `assert!(!result.is_empty())` when specific values can be verified.
3. **Thin Vertical Slices:** Implement code in small, verifiable chunks rather than massive multi-file changes.
4. **Clean Test State:** Tests must isolate and clean up temporary state, mock fixtures, and child processes after execution.
