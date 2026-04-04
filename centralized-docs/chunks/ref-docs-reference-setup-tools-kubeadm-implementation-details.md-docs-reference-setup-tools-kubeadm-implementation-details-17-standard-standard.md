---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#17-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 337
summary: * `--requestheader-username-headers=X-Remote-User` * `--requestheader-group-headers=X-Remote-Group` * `--requestheader-extra-headers-prefix=X-Remote-Extra-` *...
---

* `--requestheader-username-headers=X-Remote-User`
* `--requestheader-group-headers=X-Remote-Group`
* `--requestheader-extra-headers-prefix=X-Remote-Extra-`
* `--requestheader-allowed-names=front-proxy-client`#### Controller manager
The static Pod manifest for the controller manager is affected by following parameters provided by
the users:
* If kubeadm is invoked specifying a `--pod-network-cidr`, the subnet manager feature required for
some CNI network plugins is enabled by setting:
* `--allocate-node-cidrs=true`
* `--cluster-cidr` and `--node-cidr-mask-size` flags according to the given CIDR
Other flags that are set unconditionally are:
* `--controllers` enabling all the default controllers plus `BootstrapSigner` and `TokenCleaner`
controllers for TLS bootstrap. See [TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/)
for more details.
* `--use-service-account-credentials` to `true`
* Flags for using certificates generated in previous steps:
* `--root-ca-file` to `ca.crt`
* `--cluster-signing-cert-file` to `ca.crt`, if External CA mode is disabled, otherwise to `""`
* `--cluster-signing-key-file` to `ca.key`, if External CA mode is disabled, otherwise to `""`
* `--service-account-private-key-file` to `sa.key`#### Scheduler
The static Pod manifest for the scheduler is not affected by parameters provided by the user.