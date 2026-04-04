---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#82-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 107
summary: * The credentials (`ca.crt` and `token`) to the control plane come from the ServiceAccount * The location (URL) of the API server comes from a ConfigMap * The `kube-proxy` ServiceAccount is bound to...
---

* The credentials (`ca.crt` and `token`) to the control plane come from the ServiceAccount
* The location (URL) of the API server comes from a ConfigMap
* The `kube-proxy` ServiceAccount is bound to the privileges in the `system:node-proxier` ClusterRole#### DNS
* The CoreDNS service is named `kube-dns` for compatibility reasons with the legacy `kube-dns`
addon.
* A ServiceAccount for CoreDNS is created in the `kube-system` namespace.