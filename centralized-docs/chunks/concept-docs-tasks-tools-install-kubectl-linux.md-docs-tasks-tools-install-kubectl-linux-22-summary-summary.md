---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#22-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 66
summary: 1. Install kubectl using `yum`: ``` `sudo yum install -y kubectl ` ``` 1. Add the Kubernetes `zypper` repository. If you want to use Kubernetes version different than v1.35, replace v1.35 with the...
---

1. Install kubectl using `yum`:
```
`sudo yum install -y kubectl
`
```
1. Add the Kubernetes `zypper` repository. If you want to use Kubernetes version
different than v1.35, replace v1.35 with
the desired minor version in the command below.