---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#38-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 124
summary: * A kubeconfig file for scheduler, `/etc/kubernetes/scheduler.conf`; inside this file is embedded a client certificate with scheduler identity. This client certificate should have the CN...
---

* A kubeconfig file for scheduler, `/etc/kubernetes/scheduler.conf`; inside this file is embedded
a client certificate with scheduler identity.
This client certificate should have the CN `system:kube-scheduler`, as defined by default
[RBAC core components roles](/docs/reference/access-authn-authz/rbac/#core-component-roles)
Additionally, a kubeconfig file for kubeadm as an administrative entity is generated and stored
in `/etc/kubernetes/admin.conf`. This file includes a certificate with
`Subject: O = kubeadm:cluster-admins, CN = kubernetes-admin`.