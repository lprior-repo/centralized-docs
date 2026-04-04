---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#11-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 105
summary: And for macOS on Apple Silicon, type: ``` `curl -LO \"https://dl.k8s.io/release/v1.35.0/bin/darwin/arm64/kubectl\" ` ``` 2. Validate the binary (optional) Download the kubectl checksum file: ``` ` curl...
---

And for macOS on Apple Silicon, type:
```
`curl -LO "https://dl.k8s.io/release/v1.35.0/bin/darwin/arm64/kubectl"
`
```
2. Validate the binary (optional)
Download the kubectl checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl.sha256"
`
```