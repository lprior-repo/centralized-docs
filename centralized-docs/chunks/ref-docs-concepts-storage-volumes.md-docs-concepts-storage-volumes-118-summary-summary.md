---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#118-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 120
summary: CSI driver components to identify which PV objects belong to the CSI driver. * `volumeHandle`: A string value that uniquely identifies the volume. This value must correspond to the value returned in...
---

CSI driver components to identify which PV objects belong to the CSI driver.
* `volumeHandle`: A string value that uniquely identifies the volume. This value
must correspond to the value returned in the `volume.id` field of the
`CreateVolumeResponse` by the CSI driver as defined in the
[CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#createvolume).
The value is passed as `volume\_id` in all calls to the CSI volume driver when
referencing the volume.
* `readOnly`: An optional boolean value indicating whether the volume is to be