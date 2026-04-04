---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#99-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 123
summary: #### Note: Make sure you have an existing PortworxVolume with the name `pxvol` before using it in the Pod. #### Portworx CSI migration FEATURE STATE: `Kubernetes v1.33 [stable]`(enabled by default)...
---

#### Note:
Make sure you have an existing PortworxVolume with the name `pxvol`
before using it in the Pod.
#### Portworx CSI migration
FEATURE STATE:
`Kubernetes v1.33 [stable]`(enabled by default)
In Kubernetes 1.35, all operations for the in-tree
Portworx volumes are redirected to the `pxd.portworx.com`
Container Storage Interface (CSI) Driver by default.
[Portworx CSI Driver](https://docs.portworx.com/portworx-enterprise/operations/operate-kubernetes/storage-operations/csi)