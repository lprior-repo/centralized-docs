---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#0-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 268
summary: ## Table of Contents    - [Objectives](#objectives)   - [Kubernetes Deployments](#kubernetes-deployments)       - [Note:](#note)   - [Deploying your first app on...
---

## Table of Contents

  - [Objectives](#objectives)
  - [Kubernetes Deployments](#kubernetes-deployments)
      - [Note:](#note)
  - [Deploying your first app on Kubernetes](#deploying-your-first-app-on-kubernetes)
    - [kubectl basics](#kubectl-basics)
    - [Deploy an app](#deploy-an-app)
    - [View the app](#view-the-app)
      - [Note:](#note)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Objectives
* Learn about application Deployments.
* Deploy your first app on Kubernetes with kubectl.## Before you begin
The shell commands in this tutorial use POSIX shell syntax, which is supported by
the default shells on most Linux and macOS systems (for example, bash, zsh, or sh).
Windows users must use a POSIX-compatible shell such as
[Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)
or [Git Bash](https://gitforwindows.org/) to run the commands as written.
Commands that use `export`, `$()`, and similar constructs are **not** compatible
with PowerShell or the Windows Command Prompt.