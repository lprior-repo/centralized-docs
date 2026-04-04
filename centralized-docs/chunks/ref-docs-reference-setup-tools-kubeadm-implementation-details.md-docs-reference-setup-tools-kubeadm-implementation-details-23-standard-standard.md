---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#23-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 119
summary: ## kubeadm join phases internal design Similarly to `kubeadm init`, also `kubeadm join` internal workflow consists of a sequence of atomic work tasks to perform. This is split into discovery (having...
---

## kubeadm join phases internal design
Similarly to `kubeadm init`, also `kubeadm join` internal workflow consists of a sequence of
atomic work tasks to perform.
This is split into discovery (having the Node trust the Kubernetes API Server) and TLS bootstrap
(having the Kubernetes API Server trust the Node).
see [Authenticating with Bootstrap Tokens](/docs/reference/access-authn-authz/bootstrap-tokens/)
or the corresponding [design proposal](https://git.k8s.io/design-proposals-archive/cluster-lifecycle/bootstrap-discovery.md).