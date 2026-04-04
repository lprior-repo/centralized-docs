---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#52-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 125
summary: * If a cloud provider is specified, the corresponding `--cloud-provider` parameter is configured together with the `--cloud-config` path if such file exists (this is experimental, alpha and will be...
---

* If a cloud provider is specified, the corresponding `--cloud-provider` parameter is configured together
with the `--cloud-config` path if such file exists (this is experimental, alpha and will be
removed in a future version)
Other API server flags that are set unconditionally are:
* `--insecure-port=0` to avoid insecure connections to the api server
* `--enable-bootstrap-token-auth=true` to enable the `BootstrapTokenAuthenticator` authentication module.
See [TLS Bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/) for more details