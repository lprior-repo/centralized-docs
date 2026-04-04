---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#41-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 106
summary: The `super-admin.conf` file must be stored in a safe location and should not be shared with additional users. See [RBAC user facing role...
---

The `super-admin.conf` file must be stored in a safe location and should not be shared with additional users.
See [RBAC user facing role bindings](/docs/reference/access-authn-authz/rbac/#user-facing-roles)
for additional information on RBAC and built-in ClusterRoles and groups.
You can run [`kubeadm kubeconfig user`](/docs/reference/setup-tools/kubeadm/kubeadm-kubeconfig/#cmd-kubeconfig-user)
to generate kubeconfig files for additional users.