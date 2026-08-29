---
name: search
description: High performance local workspace documentation and keyword retrieval engine.
version: 1.0.0
triggers:
  - "search docs"
  - "find documentation"
  - "search workspace"
  - "lookup keyword"
execution_mode: auto
default_turns: 1
---

# 🔍 Local Documentation & Search Plugin

This plugin indexes and searches local workspace markdown, source code comments, and technical documentation offline with sub-millisecond retrieval.

## 📋 Input Payload Schema (JSON)
```json
{
  "query": "search query string",
  "limit": 5
}
```

## ⚡ Triggers
Emit `<TRIGGER:plugin:search>{"query": "<search_term>"}</TRIGGER>` to search local project resources.
