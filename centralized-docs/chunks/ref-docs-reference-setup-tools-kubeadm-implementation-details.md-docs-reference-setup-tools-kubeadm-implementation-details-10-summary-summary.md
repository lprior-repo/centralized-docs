---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#10-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 124
summary: * **Secure**: It should adopt latest best-practices like: * enforcing RBAC * using the Node Authorizer * using secure communication between the control plane components * using secure communication...
---

* **Secure**: It should adopt latest best-practices like:
* enforcing RBAC
* using the Node Authorizer
* using secure communication between the control plane components
* using secure communication between the API server and the kubelets
* lock-down the kubelet API
* locking down access to the API for system components like the kube-proxy and CoreDNS
* locking down what a Bootstrap Token can access
* **User-friendly**: The user should not have to run anything more than a couple of commands:
* `kubeadm init`
* `export KUBECONFIG=/etc/kubernetes/admin.conf`