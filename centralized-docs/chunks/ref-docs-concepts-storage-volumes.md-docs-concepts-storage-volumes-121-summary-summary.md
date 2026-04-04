---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#121-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 118
summary: This map must correspond to the map returned in the `volume.attributes` field of the `CreateVolumeResponse` by the CSI driver as defined in the [CSI...
---

This map must correspond to the map returned in the
`volume.attributes` field of the `CreateVolumeResponse` by the CSI driver as
defined in the [CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#createvolume).
The map is passed to the CSI driver via the `volume\_context` field in the
`ControllerPublishVolumeRequest`, `NodeStageVolumeRequest`, and
`NodePublishVolumeRequest`.
* `controllerPublishSecretRef`: A reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI