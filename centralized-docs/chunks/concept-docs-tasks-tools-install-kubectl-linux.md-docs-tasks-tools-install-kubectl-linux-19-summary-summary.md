---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#19-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 61
summary: ``` `sudo apt-get update sudo apt-get install -y kubectl ` ``` 1. Add the Kubernetes `yum` repository. If you want to use Kubernetes version different than v1.35, replace v1.35 with the desired minor...
---

```
`sudo apt-get update
sudo apt-get install -y kubectl
`
```
1. Add the Kubernetes `yum` repository. If you want to use Kubernetes version
different than v1.35, replace v1.35 with
the desired minor version in the command below.