---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#117-summary
chunk_level: summary
chunk_type: prose
heading: Out-of-tree volume plugins
token_count: 114
summary: persistent volume: * `driver`: A string value that specifies the name of the volume driver to use. This value must correspond to the value returned in the `GetPluginInfoResponse` by the CSI driver as...
---

persistent volume:
* `driver`: A string value that specifies the name of the volume driver to use.
This value must correspond to the value returned in the `GetPluginInfoResponse`
by the CSI driver as defined in the
[CSI spec](https://github.com/container-storage-interface/spec/blob/master/spec.md#getplugininfo).
It is used by Kubernetes to identify which CSI driver to call out to, and by
CSI driver components to identify which PV objects belong to the CSI driver.
* `volumeHandle`: A string value that uniquely identifies the volume. This value