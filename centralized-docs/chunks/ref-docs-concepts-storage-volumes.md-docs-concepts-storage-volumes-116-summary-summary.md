---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#116-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 114
summary: * through a reference to a [PersistentVolumeClaim](#persistentvolumeclaim) * with a [generic ephemeral volume](/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes) * with a [CSI...
---

* through a reference to a [PersistentVolumeClaim](#persistentvolumeclaim)
* with a [generic ephemeral volume](/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes)
* with a [CSI ephemeral volume](/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)
if the driver supports that
The following fields are available to storage administrators to configure a CSI
persistent volume:
* `driver`: A string value that specifies the name of the volume driver to use.