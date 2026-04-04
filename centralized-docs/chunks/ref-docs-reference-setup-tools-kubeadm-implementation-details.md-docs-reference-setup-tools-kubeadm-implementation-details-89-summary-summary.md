---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#89-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 63
summary: * First, the CA certificate is retrieved via insecure connection (this is possible because `kubeadm init` is granted access to `cluster-info` users for `system:unauthenticated`) * Then the CA...
---

* First, the CA certificate is retrieved via insecure connection (this is possible because
`kubeadm init` is granted access to `cluster-info` users for `system:unauthenticated`)
* Then the CA certificate goes through following validation steps:
* Basic validation: using the token ID against a JWT signature