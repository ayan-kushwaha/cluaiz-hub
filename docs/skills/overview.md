# 🧠 Skills: Cognitive Architecture & Context Injection

In Cluaiz, **Skills** represent the **Cognitive Brain** of the agentic system. While Plugins and MCP servers provide computational muscle (binaries and APIs), a Skill provides the neural reasoning frameworks, domain rules, and prompt guardrails that guide the LLM.

---

## 🎯 1. What is a Skill?

A Skill is a declarative package containing:
- **`manifest-skill.yaml`**: The Engine Skill Router manifest defining metadata, semantic triggers, and dependencies.
- **`SKILL.md`**: The structured prompt instructions dynamically injected into the LLM system context.
- **`assets/icon.svg`**: A pure SVG vector icon for native UI rendering.
- **`package.json`**: Package distribution metadata for Cluaiz Hub.

```
skills/my-skill/
├── manifest-skill.yaml  ← Router triggers & permissions
├── SKILL.md             ← Neural instructions & domain protocol
├── package.json         ← Hub registry packaging
└── assets/
    └── icon.svg         ← Vector UI icon
```

---

## ⚡ 2. Dynamic KV-Cache Context Injection

Traditional AI applications suffer from the **"God Prompt" Anti-Pattern**—dumping thousands of lines of prompt instructions into the LLM context at boot time. This consumes massive amounts of VRAM and degrades attention accuracy.

### The Cluaiz Solution: JIT Context Injection
1. **Dormant by Default:** Skills occupy zero VRAM when inactive.
2. **Semantic Activation:** When a user's prompt matches a skill's `discovery.semantic_triggers`, the `SkillRouter` activates the skill.
3. **JIT Prompt Injection:** The contents of `SKILL.md` are injected into the active turn context.
4. **Auto-Purge:** Upon turn completion, the prompt is purged from the context window, keeping inference fast and memory-efficient.

---

## 🛡️ 3. Execution Constraints & Guardrails

- **Zero Binary Overhead:** Skills never execute foreign machine code directly.
- **Dependency Binding:** A Skill can declare dependencies on underlying Plugins (`manifest-skill.yaml -> dependencies.plugins: ["math"]`) to instruct the LLM on which execution muscle to trigger.
- **Determinism:** Skills must be written with unambiguous, structured protocol rules to prevent LLM hallucinations.
