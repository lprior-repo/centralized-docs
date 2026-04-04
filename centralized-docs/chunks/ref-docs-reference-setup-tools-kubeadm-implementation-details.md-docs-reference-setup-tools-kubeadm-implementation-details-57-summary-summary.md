---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#57-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 119
summary: * `--kubelet-preferred-address-types` to `InternalIP,ExternalIP,Hostname;` this makes `kubectl logs` and other API server-kubelet communication work in environments where the hostnames of the nodes...
---

* `--kubelet-preferred-address-types` to `InternalIP,ExternalIP,Hostname;` this makes `kubectl logs` and other API server-kubelet communication work in environments where the hostnames of the
nodes aren't resolvable
* Flags for using certificates generated in previous steps:
* `--client-ca-file` to `ca.crt`
* `--tls-cert-file` to `apiserver.crt`
* `--tls-private-key-file` to `apiserver.key`
* `--kubelet-client-certificate` to `apiserver-kubelet-client.crt`