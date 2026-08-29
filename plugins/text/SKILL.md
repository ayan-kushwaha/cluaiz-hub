---
name: text
description: Deterministic string transformation, regex extraction, hashing, and diffing engine.
version: 1.0.0
triggers:
  - "regex extract"
  - "sha256 hash"
  - "base64 encode"
  - "base64 decode"
  - "diff text"
execution_mode: auto
default_turns: 1
---

# 📝 Text & Cryptographic Transformation Plugin

This plugin provides exact string manipulation, pattern matching, cryptographic hashing (SHA256, MD5), base64 encoding/decoding, and structural diffs without LLM transcription errors.

## 📋 Input Payload Schema (JSON)
```json
{
  "action": "hash" | "regex" | "base64_encode" | "base64_decode" | "diff",
  "input": "string to process",
  "pattern": "optional regex pattern",
  "target": "optional diff comparison string"
}
```

## ⚡ Triggers
Emit `<TRIGGER:plugin:text>{"action": "sha256", "input": "text"}</TRIGGER>` for deterministic data hashing or string operations.
