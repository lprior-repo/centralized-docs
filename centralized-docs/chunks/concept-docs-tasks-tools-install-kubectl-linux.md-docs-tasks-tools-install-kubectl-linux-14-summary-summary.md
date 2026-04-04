---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#14-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 112
summary: ### Install using native package management 1. Update the `apt` package index and install packages needed to use the Kubernetes `apt` repository: ``` `sudo apt-get update # apt-transport-https may be...
---

### Install using native package management
1. Update the `apt` package index and install packages needed to use the Kubernetes `apt` repository:
```
`sudo apt-get update
# apt-transport-https may be a dummy package; if so, you can skip that package
sudo apt-get install -y apt-transport-https ca-certificates curl gnupg
`
```
2. Download the public signing key for the Kubernetes package repositories. The same signing key is used for all repositories so you can disregard the version in the URL: