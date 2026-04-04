---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#12-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 491
summary: * Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;` * A kubeconfig file for controller-manager, `/etc/kubernetes/controller-manager.conf`; inside this file is embedded a client...
---

* Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;`
* A kubeconfig file for controller-manager, `/etc/kubernetes/controller-manager.conf`; inside this
file is embedded a client certificate with controller-manager identity. This client certificate should
have the CN `system:kube-controller-manager`, as defined by default
[RBAC core components roles](/docs/reference/access-authn-authz/rbac/#core-component-roles)
* A kubeconfig file for scheduler, `/etc/kubernetes/scheduler.conf`; inside this file is embedded
a client certificate with scheduler identity.
This client certificate should have the CN `system:kube-scheduler`, as defined by default
[RBAC core components roles](/docs/reference/access-authn-authz/rbac/#core-component-roles)
Additionally, a kubeconfig file for kubeadm as an administrative entity is generated and stored
in `/etc/kubernetes/admin.conf`. This file includes a certificate with
`Subject: O = kubeadm:cluster-admins, CN = kubernetes-admin`. `kubeadm:cluster-admins`
is a group managed by kubeadm. It is bound to the `cluster-admin` ClusterRole during `kubeadm init`,
by using the `super-admin.conf` file, which does not require RBAC.
This `admin.conf` file must remain on control plane nodes and should not be shared with additional users.
During `kubeadm init` another kubeconfig file is generated and stored in `/etc/kubernetes/super-admin.conf`.
This file includes a certificate with `Subject: O = system:masters, CN = kubernetes-super-admin`.
`system:masters` is a superuser group that bypasses RBAC and makes `super-admin.conf` useful in case
of an emergency where a cluster is locked due to RBAC misconfiguration.
The `super-admin.conf` file must be stored in a safe location and should not be shared with additional users.
See [RBAC user facing role bindings](/docs/reference/access-authn-authz/rbac/#user-facing-roles)
for additional information on RBAC and built-in ClusterRoles and groups.
You can run [`kubeadm kubeconfig user`](/docs/reference/setup-tools/kubeadm/kubeadm-kubeconfig/#cmd-kubeconfig-user)
to generate kubeconfig files for additional users.