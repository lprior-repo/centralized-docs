---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#11-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 116
summary: ``` ` curl -LO \"https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl.sha256\" ` ``` Validate the kubectl binary against the checksum file: ``` `echo...
---

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