---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#91-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 76
summary: * As an additional validation, the CA certificate is retrieved via secure connection and then compared with the CA retrieved initially #### Note: You can skip CA validation by passing the...
---

* As an additional validation, the CA certificate is retrieved via secure connection and then
compared with the CA retrieved initially
#### Note:
You can skip CA validation by passing the `--discovery-token-unsafe-skip-ca-verification` flag on the command line.
This weakens the kubeadm security model since others can potentially impersonate the Kubernetes API server.