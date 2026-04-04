---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#24-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 110
summary: ### Call \"kubeadm upgrade\" **For the first control plane node** 1. Upgrade kubeadm: ``` `# replace x in 1.35.x-\* with the latest patch version sudo apt-mark unhold kubeadm &amp;&amp; \\ sudo apt-get...
---

### Call "kubeadm upgrade"
**For the first control plane node**
1. Upgrade kubeadm:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo apt-mark unhold kubeadm &amp;&amp; \\
sudo apt-get update &amp;&amp; sudo apt-get install -y kubeadm='1.35.x-\*' &amp;&amp; \\
sudo apt-mark hold kubeadm
`
```
For systems with DNF: