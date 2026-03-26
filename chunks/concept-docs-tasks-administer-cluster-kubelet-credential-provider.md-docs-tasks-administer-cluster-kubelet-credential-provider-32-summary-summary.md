---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#32-summary
chunk_level: summary
chunk_type: prose
heading: Service Account Token for Image Pulls
token_count: 93
summary: * Avoid needing a kubelet/node-based identity to pull images from a registry. * Allow workloads to pull images based on their own runtime identity without long-lived/persisted secrets.## Before you...
---

* Avoid needing a kubelet/node-based identity to pull images from a registry.
* Allow workloads to pull images based on their own runtime identity
without long-lived/persisted secrets.## Before you begin
* You need a Kubernetes cluster with nodes that support kubelet credential
provider plugins. This support is available in Kubernetes 1.35;
Kubernetes v1.24 and v1.25 included this as a beta feature, enabled by default.