---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#16-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 81
summary: #### Note: In releases older than Debian 12 and Ubuntu 22.04, folder `/etc/apt/keyrings` does not exist by default, and it should be created before the curl command. 1. Add the appropriate Kubernetes...
---

#### Note:
In releases older than Debian 12 and Ubuntu 22.04, folder `/etc/apt/keyrings` does not exist by default, and it should be created before the curl command.
1. Add the appropriate Kubernetes `apt` repository. If you want to use Kubernetes version different than v1.35,
replace v1.35 with the desired minor version in the command below: