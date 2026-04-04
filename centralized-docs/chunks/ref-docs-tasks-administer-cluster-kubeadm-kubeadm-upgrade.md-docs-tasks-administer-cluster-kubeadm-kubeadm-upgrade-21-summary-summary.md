---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#21-summary
chunk_level: summary
chunk_type: prose
heading: Determine which version to upgrade to
token_count: 128
summary: For systems with DNF: ``` `# Find the latest 1.35 version in the list. # It should look like 1.35.x-\*, where x is the latest patch. sudo yum list --showduplicates kubeadm...
---

For systems with DNF:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo yum list --showduplicates kubeadm --disableexcludes=kubernetes
`
```
For systems with DNF5:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo yum list --showduplicates kubeadm --setopt=disable\_excludes=kubernetes
`
```