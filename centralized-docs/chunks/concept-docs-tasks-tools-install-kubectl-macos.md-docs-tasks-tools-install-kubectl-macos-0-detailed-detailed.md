---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 386
summary: ## Table of Contents    - [Before you begin](#before-you-begin)   - [Install kubectl on macOS](#install-kubectl-on-macos)       - [Note:](#note)       - [Note:](#note)       - [Note:](#note)     -...
---

## Table of Contents

  - [Before you begin](#before-you-begin)
  - [Install kubectl on macOS](#install-kubectl-on-macos)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
    - [Install with Homebrew on macOS](#install-with-homebrew-on-macos)
    - [Install with Macports on macOS](#install-with-macports-on-macos)
  - [Verify kubectl configuration](#verify-kubectl-configuration)
    - [Troubleshooting the 'No Auth Provider Found' error message](#troubleshooting-the-no-auth-provider-found-error-message)
    - [Introduction](#introduction)
      - [Warning:](#warning)
    - [Upgrade Bash](#upgrade-bash)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
    - [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)
      - [Note:](#note)
      - [Note:](#note)
    - [Uninstall kubectl on macOS](#uninstall-kubectl-on-macos)
    - [Uninstall kubectl using the command-line](#uninstall-kubectl-using-the-command-line)
    - [Uninstall kubectl using homebrew](#uninstall-kubectl-using-homebrew)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Before you begin
You must use a kubectl version that is within one minor version difference of
your cluster. For example, a v1.35 client can communicate
with v1.34, v1.35,
and v1.36 control planes.
Using the latest compatible version of kubectl helps avoid unforeseen issues.