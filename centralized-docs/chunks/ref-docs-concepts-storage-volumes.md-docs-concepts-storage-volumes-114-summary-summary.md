---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#114-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 119
summary: ### csi [Container Storage Interface](https://github.com/container-storage-interface/spec/blob/master/spec.md) (CSI) defines a standard interface for container orchestration systems (like Kubernetes)...
---

### csi
[Container Storage Interface](https://github.com/container-storage-interface/spec/blob/master/spec.md)
(CSI) defines a standard interface for container orchestration systems (like
Kubernetes) to expose arbitrary storage systems to their container workloads.
Please read the [CSI design proposal](https://git.k8s.io/design-proposals-archive/storage/container-storage-interface.md)
for more information.
#### Note:
Support for CSI spec versions 0.2 and 0.3 is deprecated in Kubernetes
v1.13 and will be removed in a future release.
#### Note: