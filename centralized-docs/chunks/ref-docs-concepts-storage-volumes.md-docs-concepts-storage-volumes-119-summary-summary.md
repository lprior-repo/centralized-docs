---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#119-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 121
summary: referencing the volume. * `readOnly`: An optional boolean value indicating whether the volume is to be \"ControllerPublished\" (attached) as read-only. Default is false. This value is passed to the CSI...
---

referencing the volume.
* `readOnly`: An optional boolean value indicating whether the volume is to be
"ControllerPublished" (attached) as read-only. Default is false. This value is passed
to the CSI driver via the `readonly` field in the `ControllerPublishVolumeRequest`.
* `fsType`: If the PV's `VolumeMode` is `Filesystem`, then this field may be used
to specify the filesystem that should be used to mount the volume. If the
volume has not been formatted and formatting is supported, this value will be
used to format the volume.