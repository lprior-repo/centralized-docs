---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#53-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 79
summary: * `--allow-privileged` to `true` (required e.g. by kube proxy) * `--requestheader-client-ca-file` to `front-proxy-ca.crt` * `--enable-admission-plugins` to: *...
---

* `--allow-privileged` to `true` (required e.g. by kube proxy)
* `--requestheader-client-ca-file` to `front-proxy-ca.crt`
* `--enable-admission-plugins` to:
* [`NamespaceLifecycle`](/docs/reference/access-authn-authz/admission-controllers/#namespacelifecycle)
e.g. to avoid deletion of system reserved namespaces