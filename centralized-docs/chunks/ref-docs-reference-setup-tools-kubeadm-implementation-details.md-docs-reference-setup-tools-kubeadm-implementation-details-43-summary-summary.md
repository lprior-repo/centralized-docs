---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#43-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 118
summary: 1. `ca.crt` certificate is embedded in all the kubeconfig files. 2. If a given kubeconfig file exists, and its content is evaluated as compliant with the above specs, the existing file will be used...
---

1. `ca.crt` certificate is embedded in all the kubeconfig files.
2. If a given kubeconfig file exists, and its content is evaluated as compliant with the above specs,
the existing file will be used and the generation phase for the given kubeconfig will be skipped
3. If kubeadm is running in [ExternalCA mode](/docs/reference/setup-tools/kubeadm/kubeadm-init/#external-ca-mode),
all the required kubeconfig must be provided by the user as well, because kubeadm cannot
generate any of them by itself