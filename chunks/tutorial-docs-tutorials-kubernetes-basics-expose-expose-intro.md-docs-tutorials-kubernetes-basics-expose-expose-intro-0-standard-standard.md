---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#0-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 287
summary: ## Table of Contents    - [Objectives](#objectives)   - [Overview of Kubernetes Services](#overview-of-kubernetes-services)   - [Services and Labels](#services-and-labels)     - [Step 1: Creating a...
---

## Table of Contents

  - [Objectives](#objectives)
  - [Overview of Kubernetes Services](#overview-of-kubernetes-services)
  - [Services and Labels](#services-and-labels)
    - [Step 1: Creating a new Service](#step-1-creating-a-new-service)
      - [Note:](#note)
    - [Step 2: Using labels](#step-2-using-labels)
    - [Step 3: Deleting a service](#step-3-deleting-a-service)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Objectives
* Learn about a Service in Kubernetes.
* Understand how labels and selectors relate to a Service.
* Expose an application outside a Kubernetes cluster.## Before you begin
The shell commands in this tutorial use POSIX shell syntax, which is supported by
the default shells on most Linux and macOS systems (for example, bash, zsh, or sh).
Windows users must use a POSIX-compatible shell such as
[Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)
or [Git Bash](https://gitforwindows.org/) to run the commands as written.
Commands that use `export`, `$()`, and similar constructs are **not** compatible
with PowerShell or the Windows Command Prompt.