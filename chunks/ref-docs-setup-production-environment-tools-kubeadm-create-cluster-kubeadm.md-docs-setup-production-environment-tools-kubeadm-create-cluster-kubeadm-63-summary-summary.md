---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#63-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 79
summary: ``` `kubectl label nodes --all node.kubernetes.io/exclude-from-external-load-balancers- ` ``` ### Adding more control plane nodes See [Creating Highly Available Clusters with...
---

```
`kubectl label nodes --all node.kubernetes.io/exclude-from-external-load-balancers-
`
```
### Adding more control plane nodes
See [Creating Highly Available Clusters with kubeadm](/docs/setup/production-environment/tools/kubeadm/high-availability/)
for steps on creating a high availability kubeadm cluster by adding more control plane nodes.