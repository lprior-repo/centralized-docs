---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#10-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 91
summary: ``` `curl -LO https://dl.k8s.io/release/v1.35.0/bin/linux/arm64/kubectl ` ``` 2. Validate the binary (optional) Download the kubectl checksum file: ``` ` curl -LO \"https://dl.k8s.io/release/$(curl -L...
---

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