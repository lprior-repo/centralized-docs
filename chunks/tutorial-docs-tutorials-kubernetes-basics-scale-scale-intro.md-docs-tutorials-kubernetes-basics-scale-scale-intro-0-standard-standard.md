---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#0-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 247
summary: ## Table of Contents    - [Objectives](#objectives)   - [Scaling an application](#scaling-an-application)       - [Note:](#note)   - [Scaling overview](#scaling-overview)     - [Scaling a...
---

## Table of Contents

  - [Objectives](#objectives)
  - [Scaling an application](#scaling-an-application)
      - [Note:](#note)
  - [Scaling overview](#scaling-overview)
    - [Scaling a Deployment](#scaling-a-deployment)
    - [Load Balancing](#load-balancing)
      - [Note:](#note)
    - [Scale Down](#scale-down)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Objectives
* Scale an existing app manually using kubectl.## Before you begin
The shell commands in this tutorial use POSIX shell syntax, which is supported by
the default shells on most Linux and macOS systems (for example, bash, zsh, or sh).
Windows users must use a POSIX-compatible shell such as
[Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)
or [Git Bash](https://gitforwindows.org/) to run the commands as written.
Commands that use `export`, `$()`, and similar constructs are **not** compatible
with PowerShell or the Windows Command Prompt.