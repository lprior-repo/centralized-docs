---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#40-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 113
summary: During `kubeadm init` another kubeconfig file is generated and stored in `/etc/kubernetes/super-admin.conf`. This file includes a certificate with `Subject: O = system:masters, CN =...
---

During `kubeadm init` another kubeconfig file is generated and stored in `/etc/kubernetes/super-admin.conf`.
This file includes a certificate with `Subject: O = system:masters, CN = kubernetes-super-admin`.
`system:masters` is a superuser group that bypasses RBAC and makes `super-admin.conf` useful in case
of an emergency where a cluster is locked due to RBAC misconfiguration.
The `super-admin.conf` file must be stored in a safe location and should not be shared with additional users.
See