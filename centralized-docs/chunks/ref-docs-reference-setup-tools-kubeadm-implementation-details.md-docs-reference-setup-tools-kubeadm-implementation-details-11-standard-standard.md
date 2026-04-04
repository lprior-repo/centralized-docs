---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#11-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 180
summary: * A kubeconfig file for the kubelet to use during TLS bootstrap - `/etc/kubernetes/bootstrap-kubelet.conf`. Inside this file, there is a bootstrap-token or embedded client certificates for...
---

* A kubeconfig file for the kubelet to use during TLS bootstrap -
`/etc/kubernetes/bootstrap-kubelet.conf`. Inside this file, there is a bootstrap-token or embedded
client certificates for authenticating this node with the cluster.
This client certificate should:
* Be in the `system:nodes` organization, as required by the
[Node Authorization](/docs/reference/access-authn-authz/node/) module
* Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;`
* A kubeconfig file for controller-manager, `/etc/kubernetes/controller-manager.conf`; inside this
file is embedded a client certificate with controller-manager identity. This client certificate should
have the CN `system:kube-controller-manager`, as defined by default
[RBAC core components roles](/docs/reference/access-authn-authz/rbac/#core-component-roles)