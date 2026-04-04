---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#126-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 124
summary: sensitive information to pass to the CSI driver to complete the CSI `NodeStageVolume` call. This field is optional and may be empty if no secret is required. If the Secret contains more than one...
---

sensitive information to pass to the CSI driver to complete the CSI
`NodeStageVolume` call. This field is optional and may be empty if no secret
is required. If the Secret contains more than one secret, all secrets
are passed.#### CSI raw block volume support
FEATURE STATE:
`Kubernetes v1.18 [stable]`
Vendors with external CSI drivers can implement raw block volume support
in Kubernetes workloads.
You can set up your
[PersistentVolume/PersistentVolumeClaim with raw block volume support](/docs/concepts/storage/persistent-volumes/#raw-block-volume-support)