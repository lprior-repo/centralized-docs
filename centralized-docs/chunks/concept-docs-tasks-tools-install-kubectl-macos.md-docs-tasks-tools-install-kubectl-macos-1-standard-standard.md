---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#1-standard
chunk_level: standard
chunk_type: prose
heading: Install kubectl on macOS
token_count: 267
summary: ## Install kubectl on macOS The following methods exist for installing kubectl on macOS: * [Install kubectl on macOS](#install-kubectl-on-macos) * [Install kubectl binary with curl on...
---

## Install kubectl on macOS
The following methods exist for installing kubectl on macOS:
* [Install kubectl on macOS](#install-kubectl-on-macos)
* [Install kubectl binary with curl on macOS](#install-kubectl-binary-with-curl-on-macos)
* [Install with Homebrew on macOS](#install-with-homebrew-on-macos)
* [Install with Macports on macOS](#install-with-macports-on-macos)
* [Verify kubectl configuration](#verify-kubectl-configuration)
* [Optional kubectl configurations and plugins](#optional-kubectl-configurations-and-plugins)
* [Enable shell autocompletion](#enable-shell-autocompletion)
* [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)### Install kubectl binary with curl on macOS
1. Download the latest release:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl"
`
```