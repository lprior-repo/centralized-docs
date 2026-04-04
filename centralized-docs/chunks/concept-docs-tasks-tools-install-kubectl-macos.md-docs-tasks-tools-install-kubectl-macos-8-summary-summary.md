---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#8-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 95
summary: * [Enable shell autocompletion](#enable-shell-autocompletion) * [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)### Install kubectl binary with curl on macOS 1. Download the latest...
---

* [Enable shell autocompletion](#enable-shell-autocompletion)
* [Install `kubectl convert` plugin](#install-kubectl-convert-plugin)### Install kubectl binary with curl on macOS
1. Download the latest release:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
`
```