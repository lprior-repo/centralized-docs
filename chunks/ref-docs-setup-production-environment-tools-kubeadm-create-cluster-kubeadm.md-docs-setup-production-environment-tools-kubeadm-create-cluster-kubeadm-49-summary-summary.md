---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#49-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 67
summary: * By default, `kubeadm` sets up your cluster to use and enforce use of [RBAC](/docs/reference/access-authn-authz/rbac/) (role based access control). Make sure that your Pod network plugin supports...
---

* By default, `kubeadm` sets up your cluster to use and enforce use of
[RBAC](/docs/reference/access-authn-authz/rbac/) (role based access
control).
Make sure that your Pod network plugin supports RBAC, and so do any manifests
that you use to deploy it.