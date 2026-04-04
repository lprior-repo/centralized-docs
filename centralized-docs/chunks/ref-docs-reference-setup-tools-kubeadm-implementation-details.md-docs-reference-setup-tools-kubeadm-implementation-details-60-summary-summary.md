---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#60-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 123
summary: * `--allocate-node-cidrs=true` * `--cluster-cidr` and `--node-cidr-mask-size` flags according to the given CIDR Other flags that are set unconditionally are: * `--controllers` enabling all the...
---

* `--allocate-node-cidrs=true`
* `--cluster-cidr` and `--node-cidr-mask-size` flags according to the given CIDR
Other flags that are set unconditionally are:
* `--controllers` enabling all the default controllers plus `BootstrapSigner` and `TokenCleaner`
controllers for TLS bootstrap. See [TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/)
for more details.
* `--use-service-account-credentials` to `true`
* Flags for using certificates generated in previous steps: