---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#66-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 32
summary: ``` `scp root@&lt;control-plane-host&gt;:/etc/kubernetes/admin.conf . kubectl --kubeconfig ./admin.conf get nodes ` ```
---

```
`scp root@&lt;control-plane-host&gt;:/etc/kubernetes/admin.conf .
kubectl --kubeconfig ./admin.conf get nodes
`
```