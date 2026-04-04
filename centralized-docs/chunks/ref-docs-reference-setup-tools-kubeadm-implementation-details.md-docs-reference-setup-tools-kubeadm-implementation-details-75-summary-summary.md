---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#75-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 118
summary: Kubeadm ensures that the Bootstrap Token will get its CSR request automatically approved by the csrapprover controller. This is implemented by creating ClusterRoleBinding named...
---

Kubeadm ensures that the Bootstrap Token will get its CSR request automatically approved by the
csrapprover controller.
This is implemented by creating ClusterRoleBinding named `kubeadm:node-autoapprove-bootstrap`
between the `system:bootstrappers:kubeadm:default-node-token` group and the default role
`system:certificates.k8s.io:certificatesigningrequests:nodeclient`.
The role `system:certificates.k8s.io:certificatesigningrequests:nodeclient` should be created as
well, granting POST permission to