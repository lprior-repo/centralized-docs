---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#36-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 122
summary: For systems with DNF: ``` `# replace x in 1.35.x-\* with the latest patch version sudo yum install -y kubelet-'1.35.x-\*' kubectl-'1.35.x-\*' --disableexcludes=kubernetes ` ``` For systems with DNF5:...
---

For systems with DNF:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubelet-'1.35.x-\*' kubectl-'1.35.x-\*' --disableexcludes=kubernetes
`
```
For systems with DNF5:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubelet-'1.35.x-\*' kubectl-'1.35.x-\*' --setopt=disable\_excludes=kubernetes
`
```