---
title: "Dynamic Context Injection"
description: "How the Engine manages VRAM by injecting Skill contexts dynamically."
category: "Tutorials"
---

# 3. Dynamic Context Injection (VRAM Management)

A common mistake in traditional LLM applications is the "God Prompt"—dumping every single instruction, tool, and rule into the system prompt at boot time.

In cluaiz, this is an anti-pattern. Large system prompts consume massive amounts of VRAM (KV Cache) and drastically reduce response times.

## How cluaiz Solves This

cluaiz uses **Dynamic Context Injection**. 

Instead of loading every Skill into memory at once, the Engine uses the `triggers.semantic` defined in your `SKILL.md` frontmatter to route inputs.

```mermaid
flowchart TD
    A["User: 'Review my code'"] -->|Fast Embedding Match| B{"Skill Router"}
    B -->|Matches 'code', 'review'| C["Loads 'code-reviewer' Skill.md"]
    C -->|Injects to LLM Context| D["LLM Evaluates Rule"]
    D -->|Executes| E["Returns Result"]
    E -->|Purges Context| F["VRAM Freed"]
```

## Step 1: Small, Modular Skills

To take advantage of this architecture, you must write granular skills.

### Instead of this (Anti-Pattern):
`SKILL.md` (The God Prompt - 2000 lines):
*"You are an AI. If the user asks for math, do X. If they ask for search, do Y. If they ask for github, do Z."*

### Do this (Modular Skills):
**Skill 1: `math-assistant`**
`discovery.semantic_triggers: ["calculate", "math"]`
`SKILL.md`: *"Use the math plugin to calculate expressions."*

**Skill 2: `github-assistant`**
`discovery.semantic_triggers: ["pull request", "issue"]`
`SKILL.md`: *"Use the github MCP server to fetch PRs."*

## Step 2: Prefix Caching & VRAM KV-Injection (`vram_kv_inject` & `prefix_caching`)

If your skill orchestrates a powerful Plugin that generates massive amounts of data (like a database returning 50,000 rows), feeding that raw text into the LLM context will instantly cause an Out-Of-Memory (OOM) crash.

To handle this, Plugins use `vram_kv_inject: true` and `prefix_caching: true`. 
When the plugin executes, instead of returning 50,000 rows to the LLM, the Plugin injects structured context into the KV-Cache prefix. The engine runs a sub-inference on the data in chunks, and only returns the final summary delta tokens to the primary LLM session.

By structuring your Skills carefully, you ensure the Engine remains fast and VRAM-efficient.
