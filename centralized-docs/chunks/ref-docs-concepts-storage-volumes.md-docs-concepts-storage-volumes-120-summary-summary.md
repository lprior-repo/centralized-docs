---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#120-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 109
summary: If the volume has not been formatted and formatting is supported, this value will be used to format the volume. This value is passed to the CSI driver via the `VolumeCapability` field of...
---

If the
volume has not been formatted and formatting is supported, this value will be
used to format the volume.
This value is passed to the CSI driver via the `VolumeCapability` field of
`ControllerPublishVolumeRequest`, `NodeStageVolumeRequest`, and
`NodePublishVolumeRequest`.
* `volumeAttributes`: A map of string to string that specifies static properties
of a volume. This map must correspond to the map returned in the
`volume.attributes` field of the `CreateVolumeResponse` by the CSI driver as