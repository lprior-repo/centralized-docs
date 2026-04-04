---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#12-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 123
summary: ``` ` curl -LO \"https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl.sha256\" ` ``` Validate the kubectl binary against the checksum file: ``` `echo...
---

```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl.sha256"
`
```
Validate the kubectl binary against the checksum file:
```
`echo "$(cat kubectl.sha256) kubectl" | shasum -a 256 --check
`
```
If valid, the output is:
```
`kubectl: OK
`
```
If the check fails, `shasum` exits with nonzero status and prints output similar to: