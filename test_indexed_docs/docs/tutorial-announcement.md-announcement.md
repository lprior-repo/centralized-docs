---
id: tutorial/announcement.md/announcement
title: Introducing Llms.txt
category: tutorial
tags: ["format", "introducing", "llms.txt", "project", "tutorial"]
---

# Introducing llms.txt



 > 
 > **Context**: \**The Problem:** AI wastes tokens downloading entire documentation sites blindly.



**The Problem:** AI wastes tokens downloading entire documentation sites blindly.
**The Solution:** `llms.txt`, a `robots.txt` equivalent for AI agents.

By placing an `llms.txt` file at the root of a project, AI can:

* Use **60% fewer tokens**.
* Achieve **35% better accuracy**.

## Format

````yaml
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
````
## See Also

- [Documentation Index](./COMPASS.md)
