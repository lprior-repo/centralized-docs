---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#34-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 128
summary: 2. For the CA, it is possible to provide the `ca.crt` file but not the `ca.key` file. If all other certificates and kubeconfig files are already in place, kubeadm recognizes this condition and...
---

2. For the CA, it is possible to provide the `ca.crt` file but not the `ca.key` file. If all other certificates and kubeconfig files
are already in place, kubeadm recognizes this condition and activates the ExternalCA, which also implies the `csrsigner` controller in
controller-manager won't be started
3. If kubeadm is running in [external CA mode](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#external-ca-mode);
all the certificates must be provided by the user, because kubeadm cannot generate them by itself