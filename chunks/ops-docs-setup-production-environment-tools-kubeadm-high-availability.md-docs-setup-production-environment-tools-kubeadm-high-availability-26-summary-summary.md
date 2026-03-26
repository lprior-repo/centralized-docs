---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#26-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 97
summary: * The `--control-plane-endpoint` flag should be set to the address or DNS and port of the load balancer. * The `--upload-certs` flag is used to upload the certificates that should be shared across...
---

* The `--control-plane-endpoint` flag should be set to the address or DNS and port of the load balancer.
* The `--upload-certs` flag is used to upload the certificates that should be shared
across all the control-plane instances to the cluster. If instead, you prefer to copy certs across
control-plane nodes manually or using automation tools, please remove this flag and refer to [Manual
certificate distribution](#manual-certs) section below.