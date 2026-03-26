---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#40-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 107
summary: /docs/concepts/cluster-administration/addons/ You can now join any number of machines by running the following on each node as root: kubeadm join &lt;control-plane-host&gt;:&lt;control-plane-port&gt;...
---

/docs/concepts/cluster-administration/addons/
You can now join any number of machines by running the following on each node
as root:
kubeadm join &lt;control-plane-host&gt;:&lt;control-plane-port&gt; --token &lt;token&gt; --discovery-token-ca-cert-hash sha256:&lt;hash&gt;
`
```
To make kubectl work for your non-root user, run these commands, which are
also part of the `kubeadm init` output: