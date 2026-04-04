---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#1-standard
chunk_level: standard
chunk_type: code
heading: Install kubectl on Linux
token_count: 509
summary: ## Install kubectl on Linux The following methods exist for installing kubectl on Linux: * [Install kubectl binary with curl on Linux](#install-kubectl-binary-with-curl-on-linux) * [Install using...
---

## Install kubectl on Linux
The following methods exist for installing kubectl on Linux:
* [Install kubectl binary with curl on Linux](#install-kubectl-binary-with-curl-on-linux)
* [Install using native package management](#install-using-native-package-management)
* [Install using other package management](#install-using-other-package-management)### Install kubectl binary with curl on Linux
1. Download the latest release with the command:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl"
`
```
#### Note:
To download a specific version, replace the `$(curl -L -s https://dl.k8s.io/release/stable.txt)`
portion of the command with the specific version.
For example, to download version 1.35.0 on Linux x86-64, type:
```
`curl -LO https://dl.k8s.io/release/v1.35.0/bin/linux/amd64/kubectl
`
```
And for Linux ARM64, type:
```
`curl -LO https://dl.k8s.io/release/v1.35.0/bin/linux/arm64/kubectl
`
```
2. Validate the binary (optional)
Download the kubectl checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl.sha256"
`
```
Validate the kubectl binary against the checksum file:
```
`echo "$(cat kubectl.sha256) kubectl" | sha256sum --check
`
```
If valid, the output is:
```
`kubectl: OK
`
```
If the check fails, `sha256` exits with nonzero status and prints output similar to:
```
`kubectl: FAILED
sha256sum: WARNING: 1 computed checksum did NOT match
`
```