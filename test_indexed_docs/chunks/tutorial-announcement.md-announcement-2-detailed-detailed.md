---
doc_id: tutorial/announcement.md/announcement
chunk_id: tutorial/announcement.md/announcement#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Getting Started
token_count: 122
summary: # Introducing llms. **The Problem:** AI wastes tokens downloading entire documentation sites blindly
---

# Introducing llms.txt

**The Problem:** AI wastes tokens downloading entire documentation sites blindly.
**The Solution:** `llms.txt`, a `robots.txt` equivalent for AI agents.

By placing an `llms.txt` file at the root of a project, AI can:
- Use **60% fewer tokens**.
- Achieve **35% better accuracy**.

## Format
```yaml
---
llms_version: "1.0"
project: "Project Name"
url: "https://example.com"
updated: "2026-01-15"
---
# Project Name
> Description

## Getting Started
- [Install](./install.md): Installation guide
```
