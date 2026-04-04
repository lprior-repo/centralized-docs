---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#22-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 277
summary: ### Install addons Kubeadm installs the internal DNS server and the kube-proxy addon components via the API server. #### Note: This phase can be invoked individually with the command [`kubeadm init...
---

### Install addons
Kubeadm installs the internal DNS server and the kube-proxy addon components via the API server.
#### Note:
This phase can be invoked individually with the command
[`kubeadm init phase addon all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-addon).
#### proxy
A ServiceAccount for `kube-proxy` is created in the `kube-system` namespace; then kube-proxy is
deployed as a DaemonSet:
* The credentials (`ca.crt` and `token`) to the control plane come from the ServiceAccount
* The location (URL) of the API server comes from a ConfigMap
* The `kube-proxy` ServiceAccount is bound to the privileges in the `system:node-proxier` ClusterRole#### DNS
* The CoreDNS service is named `kube-dns` for compatibility reasons with the legacy `kube-dns`
addon.
* A ServiceAccount for CoreDNS is created in the `kube-system` namespace.
* The `coredns` ServiceAccount is bound to the privileges in the `system:coredns` ClusterRole
In Kubernetes version 1.21, support for using `kube-dns` with kubeadm was removed.
You can use CoreDNS with kubeadm even when the related Service is named `kube-dns`.