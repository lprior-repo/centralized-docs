---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#82-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 124
summary: #### Note: You must have your own iSCSI server running with the volume created before you can use it. A feature of iSCSI is that it can be mounted as read-only by multiple consumers simultaneously....
---

#### Note:
You must have your own iSCSI server running with the volume created before you can use it.
A feature of iSCSI is that it can be mounted as read-only by multiple consumers
simultaneously. This means that you can pre-populate a volume with your dataset
and then serve it in parallel from as many Pods as you need. Unfortunately,
iSCSI volumes can only be mounted by a single consumer in read-write mode.
Simultaneous writers are not allowed.
### local
A `local` volume represents a mounted local storage device such as a disk,
partition or directory.