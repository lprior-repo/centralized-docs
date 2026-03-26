---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#61-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 94
summary: ### Control plane node isolation By default, your cluster will not schedule Pods on the control plane nodes for security reasons. If you want to be able to schedule Pods on the control plane nodes,...
---

### Control plane node isolation
By default, your cluster will not schedule Pods on the control plane nodes for security
reasons. If you want to be able to schedule Pods on the control plane nodes,
for example for a single machine Kubernetes cluster, run:
```
`kubectl taint nodes --all node-role.kubernetes.io/control-plane-
`
```
The output will look something like:
```
`node "test-01" untainted
...
`
```