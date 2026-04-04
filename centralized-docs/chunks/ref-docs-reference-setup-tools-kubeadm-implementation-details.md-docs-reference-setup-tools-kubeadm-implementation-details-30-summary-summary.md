---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#30-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 75
summary: * Kubernetes DNS names, e.g. `kubernetes.default.svc.cluster.local` if `--service-dns-domain` flag value is `cluster.local`, plus default DNS names `kubernetes.default.svc`, `kubernetes.default`,...
---

* Kubernetes DNS names, e.g. `kubernetes.default.svc.cluster.local` if `--service-dns-domain`
flag value is `cluster.local`, plus default DNS names `kubernetes.default.svc`,
`kubernetes.default`, `kubernetes`
* The node-name
* The `--apiserver-advertise-address`
* Additional alternative names specified by the user