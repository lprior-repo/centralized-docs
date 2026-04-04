---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#81-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 33
summary: #### proxy A ServiceAccount for `kube-proxy` is created in the `kube-system` namespace; then kube-proxy is deployed as a DaemonSet:
---

#### proxy
A ServiceAccount for `kube-proxy` is created in the `kube-system` namespace; then kube-proxy is
deployed as a DaemonSet: