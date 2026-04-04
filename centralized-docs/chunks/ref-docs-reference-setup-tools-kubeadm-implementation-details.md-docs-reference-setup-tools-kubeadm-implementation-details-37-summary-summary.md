---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#37-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 95
summary: * Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;` * A kubeconfig file for controller-manager, `/etc/kubernetes/controller-manager.conf`; inside this file is embedded a client...
---

* Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;`
* A kubeconfig file for controller-manager, `/etc/kubernetes/controller-manager.conf`; inside this
file is embedded a client certificate with controller-manager identity. This client certificate should
have the CN `system:kube-controller-manager`, as defined by default
[RBAC core components roles](/docs/reference/access-authn-authz/rbac/#core-component-roles)