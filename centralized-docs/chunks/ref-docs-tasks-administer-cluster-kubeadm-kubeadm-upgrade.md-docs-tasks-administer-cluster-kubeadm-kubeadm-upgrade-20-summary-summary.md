---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#20-summary
chunk_level: summary
chunk_type: prose
heading: Determine which version to upgrade to
token_count: 83
summary: ## Determine which version to upgrade to Find the latest patch release for Kubernetes 1.35 using the OS package manager: ``` `# Find the latest 1.35 version in the list. # It should look like...
---

## Determine which version to upgrade to
Find the latest patch release for Kubernetes 1.35 using the OS package manager:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo apt update
sudo apt-cache madison kubeadm
`
```
For systems with DNF: