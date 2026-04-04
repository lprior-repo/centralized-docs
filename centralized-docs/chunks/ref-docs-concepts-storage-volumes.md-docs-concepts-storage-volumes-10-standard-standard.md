---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#10-standard
chunk_level: standard
chunk_type: prose
heading: Types of volumes
token_count: 133
summary: ### gcePersistentDisk (deprecated) In Kubernetes 1.35, all operations for the in-tree `gcePersistentDisk` type are redirected to the `pd.csi.storage.gke.io` [CSI](/docs/concepts/storage/volumes/#csi)...
---

### gcePersistentDisk (deprecated)
In Kubernetes 1.35, all operations for the in-tree `gcePersistentDisk` type
are redirected to the `pd.csi.storage.gke.io` [CSI](/docs/concepts/storage/volumes/#csi) driver.
The `gcePersistentDisk` in-tree storage driver was deprecated in the Kubernetes v1.17 release
and then removed entirely in the v1.28 release.
The Kubernetes project suggests that you use the
[Google Compute Engine Persistent Disk CSI](https://github.com/kubernetes-sigs/gcp-compute-persistent-disk-csi-driver)
third party storage driver instead.