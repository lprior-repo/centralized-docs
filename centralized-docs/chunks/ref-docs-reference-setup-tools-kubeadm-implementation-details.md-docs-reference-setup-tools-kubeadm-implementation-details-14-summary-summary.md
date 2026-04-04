---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#14-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 100
summary: * `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap) * `controller-manager.conf` * `scheduler.conf` * `admin.conf` for the cluster admin and kubeadm itself * `super-admin.conf` for the...
---

* `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap)
* `controller-manager.conf`
* `scheduler.conf`
* `admin.conf` for the cluster admin and kubeadm itself
* `super-admin.conf` for the cluster super-admin that can bypass RBAC
* Names of certificates and key files:
* `ca.crt`, `ca.key` for the Kubernetes certificate authority
* `apiserver.crt`, `apiserver.key` for the API server certificate