---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#52-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 47
summary: untrusted Pod, the containers in that Pod may be able to subvert the read-write host mount. Take care when using `hostPath` volumes, whether these are mounted as read-only or as read-write, because:
---

untrusted Pod, the containers in that Pod may be able to subvert the
read-write host mount.
Take care when using `hostPath` volumes, whether these are mounted as read-only
or as read-write, because: