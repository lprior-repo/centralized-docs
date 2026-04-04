---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#9-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 128
summary: #### Note: To download a specific version, replace the `$(curl -L -s https://dl.k8s.io/release/stable.txt)` portion of the command with the specific version. For example, to download version 1.35.0...
---

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