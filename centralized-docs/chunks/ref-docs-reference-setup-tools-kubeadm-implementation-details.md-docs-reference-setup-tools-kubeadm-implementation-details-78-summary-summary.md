---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#78-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 63
summary: #### Create the public cluster-info ConfigMap This phase creates the `cluster-info` ConfigMap in the `kube-public` namespace. Additionally, it creates a Role and a RoleBinding granting access to the...
---

#### Create the public cluster-info ConfigMap
This phase creates the `cluster-info` ConfigMap in the `kube-public` namespace.
Additionally, it creates a Role and a RoleBinding granting access to the ConfigMap for
unauthenticated users (i.e. users in RBAC group `system:unauthenticated`).