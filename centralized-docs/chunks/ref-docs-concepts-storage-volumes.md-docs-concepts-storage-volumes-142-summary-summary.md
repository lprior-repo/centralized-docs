---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#142-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 104
summary: A mount can be made read-only by setting the `.spec.containers[\*].volumeMounts[\*].readOnly` field to `true`. This does not make the volume itself read-only, but that specific container will not be...
---

A mount can be made read-only by setting the `.spec.containers[\*].volumeMounts[\*].readOnly`
field to `true`.
This does not make the volume itself read-only, but that specific container will
not be able to write to it.
Other containers in the Pod may mount the same volume as read-write.
On Linux, read-only mounts are not recursively read-only by default.
For example, consider a Pod that mounts the hosts `/mnt` as a `hostPath` volume. If