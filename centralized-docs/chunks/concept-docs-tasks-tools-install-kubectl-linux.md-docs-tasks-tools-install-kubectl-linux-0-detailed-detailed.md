---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 447
summary: ## Table of Contents    - [Before you begin](#before-you-begin)   - [Install kubectl on Linux](#install-kubectl-on-linux)       - [Note:](#note)       - [Note:](#note)       - [Note:](#note) - [and...
---

## Table of Contents

  - [Before you begin](#before-you-begin)
  - [Install kubectl on Linux](#install-kubectl-on-linux)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
- [and then append (or prepend) \~/.local/bin to $PATH](#and-then-append-or-prepend-localbin-to-path)
    - [Install using native package management](#install-using-native-package-management)
- [apt-transport-https may be a dummy package; if so, you can skip that package](#apt-transport-https-may-be-a-dummy-package-if-so-you-can-skip-that-package)
- [sudo mkdir -p -m 755 /etc/apt/keyrings](#sudo-mkdir--p--m-755-etcaptkeyrings)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
      - [Note:](#note)
    - [Install using other package management](#install-using-other-package-management)
  - [Verify kubectl configuration](#verify-kubectl-configuration)
    - [Troubleshooting the 'No Auth Provider Found' error message](#troubleshooting-the-no-auth-provider-found-error-message)
    - [Introduction](#introduction)
    - [Install bash-completion](#install-bash-completion)
      - [Bash](#bash)
      - [Note:](#note)
      - [Note:](#note)
    - [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)
      - [Note:](#note)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Before you begin
You must use a kubectl version that is within one minor version difference of
your cluster. For example, a v1.35 client can communicate
with v1.34, v1.35,
and v1.36 control planes.
Using the latest compatible version of kubectl helps avoid unforeseen issues.