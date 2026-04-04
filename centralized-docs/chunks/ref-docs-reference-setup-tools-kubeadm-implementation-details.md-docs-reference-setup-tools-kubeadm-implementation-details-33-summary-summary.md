---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#33-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 118
summary: 1. If a given certificate and private key pair both exist, and their content is evaluated to be compliant with the above specs, the existing files will be used and the generation phase for the given...
---

1. If a given certificate and private key pair both exist, and their content is evaluated to be compliant with the above specs, the existing files will
be used and the generation phase for the given certificate will be skipped. This means the user can, for example, copy an existing CA to
`/etc/kubernetes/pki/ca.{crt,key}`, and then kubeadm will use those files for signing the rest of the certs.
See also [using custom certificates](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#custom-certificates)