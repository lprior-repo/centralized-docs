---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#95-summary
chunk_level: summary
chunk_type: prose
heading: TLS Bootstrap
token_count: 115
summary: ## TLS Bootstrap Once the cluster info is known, the file `bootstrap-kubelet.conf` is written, thus allowing kubelet to do TLS Bootstrapping. The TLS bootstrap mechanism uses the shared token to...
---

## TLS Bootstrap
Once the cluster info is known, the file `bootstrap-kubelet.conf` is written, thus allowing
kubelet to do TLS Bootstrapping.
The TLS bootstrap mechanism uses the shared token to temporarily authenticate with the Kubernetes
API server to submit a certificate signing request (CSR) for a locally created key pair.
The request is then automatically approved and the operation completes saving `ca.crt` file and
`kubelet.conf` file to be used by the kubelet for joining the cluster, while `bootstrap-kubelet.conf`
is deleted.