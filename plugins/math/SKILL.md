---
name: math
description: Deterministic high-precision mathematical evaluation engine. Eliminates LLM calculation and arithmetic hallucination.
version: 1.0.0
triggers:
  - "calculate"
  - "solve equation"
  - "math expression"
  - "trigonometry"
  - "evaluate math"
execution_mode: auto
default_turns: 1
---

# 📐 Math Evaluation Plugin

This plugin provides exact, deterministic arithmetic, trigonometry, calculus, and large-number computations inside an isolated WASM sandbox with zero hallucination.

## 📋 Input Payload Schema (JSON)
```json
{
  "expression": "2^64 - 1",
  "precision": 10
}
```

## ⚡ Triggers
Emit `<TRIGGER:plugin:math>{"expression": "<math_expr>"}</TRIGGER>` when exact numeric precision is required.
