---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#35-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 102
summary: 1. Upgrade the kubelet and kubectl: ``` `# replace x in 1.35.x-\* with the latest patch version sudo apt-mark unhold kubelet kubectl &amp;&amp; \\ sudo apt-get update &amp;&amp; sudo apt-get install...
---

1. Upgrade the kubelet and kubectl:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo apt-mark unhold kubelet kubectl &amp;&amp; \\
sudo apt-get update &amp;&amp; sudo apt-get install -y kubelet='1.35.x-\*' kubectl='1.35.x-\*' &amp;&amp; \\
sudo apt-mark hold kubelet kubectl
`
```
For systems with DNF: