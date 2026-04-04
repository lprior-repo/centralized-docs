---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#32-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 117
summary: ### downwardAPI A `downwardAPI` volume makes [downward API](/docs/concepts/workloads/pods/downward-api/) data available to applications. Within the volume, you can find the exposed data as read-only...
---

### downwardAPI
A `downwardAPI` volume makes [downward API](/docs/concepts/workloads/pods/downward-api/)
data available to applications. Within the volume, you can find the exposed
data as read-only files in plain text format.
#### Note:
A container using the downward API as a [`subPath`](#using-subpath) volume mount does not
receive updates when field values change.
See [Expose Pod Information to Containers Through Files](/docs/tasks/inject-data-application/downward-api-volume-expose-pod-information/)
to learn more.