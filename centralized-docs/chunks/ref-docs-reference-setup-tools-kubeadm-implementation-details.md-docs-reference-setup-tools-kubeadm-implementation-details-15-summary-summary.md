---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#15-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 95
summary: * `apiserver.crt`, `apiserver.key` for the API server certificate * `apiserver-kubelet-client.crt`, `apiserver-kubelet-client.key` for the client certificate used by the API server to connect to the...
---

* `apiserver.crt`, `apiserver.key` for the API server certificate
* `apiserver-kubelet-client.crt`, `apiserver-kubelet-client.key` for the client certificate used
by the API server to connect to the kubelets securely
* `sa.pub`, `sa.key` for the key used by the controller manager when signing ServiceAccount
* `front-proxy-ca.crt`, `front-proxy-ca.key` for the front proxy certificate authority