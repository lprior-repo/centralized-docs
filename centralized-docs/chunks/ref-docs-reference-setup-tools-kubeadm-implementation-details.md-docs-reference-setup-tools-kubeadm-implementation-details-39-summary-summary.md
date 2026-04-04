---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#39-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 122
summary: . This file includes a certificate with `Subject: O = kubeadm:cluster-admins, CN = kubernetes-admin`. `kubeadm:cluster-admins` is a group managed by kubeadm. It is bound to the `cluster-admin`...
---

. This file includes a certificate with
`Subject: O = kubeadm:cluster-admins, CN = kubernetes-admin`. `kubeadm:cluster-admins`
is a group managed by kubeadm. It is bound to the `cluster-admin` ClusterRole during `kubeadm init`,
by using the `super-admin.conf` file, which does not require RBAC.
This `admin.conf` file must remain on control plane nodes and should not be shared with additional users.
During `kubeadm init` another kubeconfig file is generated and stored in