---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#58-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 128
summary: * `--kubelet-client-certificate` to `apiserver-kubelet-client.crt` * `--kubelet-client-key` to `apiserver-kubelet-client.key` * `--service-account-key-file` to `sa.pub` *...
---

* `--kubelet-client-certificate` to `apiserver-kubelet-client.crt`
* `--kubelet-client-key` to `apiserver-kubelet-client.key`
* `--service-account-key-file` to `sa.pub`
* `--requestheader-client-ca-file` to `front-proxy-ca.crt`
* `--proxy-client-cert-file` to `front-proxy-client.crt`
* `--proxy-client-key-file` to `front-proxy-client.key`
* Other flags for securing the front proxy
([API Aggregation](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/))
communications: