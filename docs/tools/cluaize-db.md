# Tutorial: Using the cluaiz-db Plugin

This tutorial guides you through using the `cluaiz-db` plugin to manage persistent local data.

## 1. Installation
Install the database plugin using the cluaiz CLI:
```bash
cluaiz plugin install cluaiz-db
```
Verify installation:
```bash
cluaiz plugin list
```

## 2. Triggering via AI
The database plugin is loaded lazily. Ask the AI to store or retrieve data:
```bash
cluaiz chat "Save my API key for GitHub as 'gh_token_123'"
```
The AI will automatically route this request using the CEL command:
`use plugin::cluaiz-db -> execute(...)`

## 3. Direct CEL Execution
You can manually execute database commands via CEL without the AI router:
```bash
cluaiz run "use plugin::cluaiz-db -> execute(action: 'set', key: 'github_token', value: 'gh_token_123')"
```
