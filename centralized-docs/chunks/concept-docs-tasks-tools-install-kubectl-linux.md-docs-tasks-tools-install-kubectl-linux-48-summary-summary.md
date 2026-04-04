---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#48-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 126
summary: 2. Validate the binary (optional) Download the kubectl-convert checksum file: ``` ` curl -LO \"https://dl.k8s.io/release/$(curl -L -s...
---

2. Validate the binary (optional)
Download the kubectl-convert checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl-convert.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl-convert.sha256"
`
```
Validate the kubectl-convert binary against the checksum file: