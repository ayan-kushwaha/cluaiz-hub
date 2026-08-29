# 🚀 Tutorial: Build Your First Cluaiz Skill

In this hands-on tutorial, we will build, package, and test a complete Skill from scratch: **`git-assistant`**.

---

## 🛠️ Step 1: Create Directory Structure

Create a new directory in your local workspace or `~/.cluaiz/tools/skills/`:

```bash
mkdir -p git-assistant/assets
cd git-assistant
```

---

## 📄 Step 2: Create `manifest-skill.yaml`

Create `manifest-skill.yaml` to define triggers and router metadata:

```yaml
name: "git-assistant"
version: "1.0.0"
description: "Assists developers with Git commit conventions, branch workflows, and merge conflict resolution."
author: "Aryan"
type: "skill"

discovery:
  semantic_triggers:
    - "git help"
    - "write commit message"
    - "git workflow"
    - "resolve merge conflict"

dependencies:
  plugins: []
  mcp:
    - "git"

permissions:
  filesystem: true
  network: false
  level: "ReadOnly"

execution:
  execution_mode: "auto"
  default_turns: 2
```

---

## ✍️ Step 3: Create `SKILL.md`

Create `SKILL.md` with full YAML frontmatter and instructional rules:

```markdown
---
id: cluaiz.skill.devops.git-assistant
name: git-assistant
version: 1.0.0
description: Assists developers with Git commit conventions, branch workflows, and merge conflict resolution.
author: Aryan
soul_type: markdown

permissions:
  filesystem: true
  network: false
  level: ReadOnly
  mcp_servers: []

triggers:
  semantic:
    - "git help"
    - "write commit message"
    - "git workflow"
    - "resolve merge conflict"
  entropy_threshold: 0.7
---

# 🌿 Git Workflow Protocol

When this skill is active, you enforce strict Git best practices:

## 1. Conventional Commits Standard
Format all commit messages strictly as: `<type>(<scope>): <subject>`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`.
- Imperative tone: "add feature" NOT "added feature".

## 2. Merge Conflict Resolution
When explaining merge conflicts, show exact 3-way conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).
```

---

## 🎨 Step 4: Add `assets/icon.svg`

Create `assets/icon.svg` for UI rendering:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#10B981" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="18" cy="18" r="3"/>
  <circle cx="6" cy="6" r="3"/>
  <circle cx="6" cy="18" r="3"/>
  <path d="M18 9a9 9 0 0 1-9 9"/>
  <line x1="6" y1="9" x2="6" y2="15"/>
</svg>
```

---

## 📦 Step 5: Create `package.json`

Create `package.json` for Cluaiz Hub distribution:

```json
{
  "name": "git-assistant",
  "version": "1.0.0",
  "description": "Assists developers with Git commit conventions, branch workflows, and merge conflict resolution",
  "category": "skill",
  "author": "Aryan",
  "license": "Apache-2.0",
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "files": {
        "manifest": "manifest-skill.yaml",
        "skill": "SKILL.md",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

---

## 🧪 Step 6: Test Locally

Copy the folder to `~/.cluaiz/tools/skills/git-assistant/`. The Cluaiz engine will automatically probe and register the new skill on the next inference query!
