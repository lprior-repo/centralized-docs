---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#77-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 102
summary: #### Set up nodes certificate rotation with auto approval Kubeadm ensures that certificate rotation is enabled for nodes, and that a new certificate request for nodes will get its CSR request...
---

#### Set up nodes certificate rotation with auto approval
Kubeadm ensures that certificate rotation is enabled for nodes, and that a new certificate request
for nodes will get its CSR request automatically approved by the csrapprover controller.
This is implemented by creating ClusterRoleBinding named
`kubeadm:node-autoapprove-certificate-rotation` between the `system:nodes` group and the default
role `system:certificates.k8s.io:certificatesigningrequests:selfnodeclient`.