---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#73-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 126
summary: 3. Additional tokens can be created with the [`kubeadm token`](/docs/reference/setup-tools/kubeadm/kubeadm-token/) command, that provide other useful functions for token management as well.#### Allow...
---

3. Additional tokens can be created with the [`kubeadm token`](/docs/reference/setup-tools/kubeadm/kubeadm-token/)
command, that provide other useful functions for token management as well.#### Allow joining nodes to call CSR API
Kubeadm ensures that users in `system:bootstrappers:kubeadm:default-node-token` group are able to
access the certificate signing API.
This is implemented by creating a ClusterRoleBinding named `kubeadm:kubelet-bootstrap` between the
group above and the default RBAC role `system:node-bootstrapper`.