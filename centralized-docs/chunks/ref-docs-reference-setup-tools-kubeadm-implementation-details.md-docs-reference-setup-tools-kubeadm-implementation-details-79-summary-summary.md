---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#79-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 79
summary: #### Note: The access to the `cluster-info` ConfigMap *is not* rate-limited. This may or may not be a problem if you expose your cluster's API server to the internet; worst-case scenario here is a...
---

#### Note:
The access to the `cluster-info` ConfigMap *is not* rate-limited. This may or may not be a
problem if you expose your cluster's API server to the internet; worst-case scenario here is a
DoS attack where an attacker uses all the in-flight requests the kube-apiserver can handle to
serve the `cluster-info` ConfigMap.