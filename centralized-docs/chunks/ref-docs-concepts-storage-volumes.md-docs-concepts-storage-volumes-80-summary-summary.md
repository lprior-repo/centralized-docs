---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#80-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 128
summary: [More info about container images](/docs/concepts/containers/images/).`pullPolicy`Policy for pulling OCI objects. Possible values are: `Always`, `Never`, or `IfNotPresent`. Defaults to `Always` if...
---

[More info about container images](/docs/concepts/containers/images/).`pullPolicy`Policy for pulling OCI objects. Possible values are: `Always`, `Never`, or
`IfNotPresent`. Defaults to `Always` if `:latest` tag is specified, or
`IfNotPresent` otherwise.
See the [*Use an Image Volume With a Pod*](/docs/tasks/configure-pod-container/image-volumes/)
example for more details on how to use the volume source.
### iscsi
An `iscsi` volume allows an existing iSCSI (SCSI over IP) volume to be mounted