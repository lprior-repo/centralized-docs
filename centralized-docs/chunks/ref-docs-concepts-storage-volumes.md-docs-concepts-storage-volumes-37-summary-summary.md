---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#37-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 126
summary: , Kubernetes mounts a tmpfs (RAM-backed filesystem) for you instead. While tmpfs is very fast, be aware that, unlike disks, files you write count against the memory limit of the container that wrote...
---

, Kubernetes mounts a tmpfs (RAM-backed
filesystem) for you instead. While tmpfs is very fast, be aware that, unlike
disks, files you write count against the memory limit of the container that wrote them.
A size limit can be specified for the default medium, which limits the capacity
of the `emptyDir` volume. The storage is allocated from
[node ephemeral storage](/docs/concepts/storage/ephemeral-storage/#setting-requests-and-limits-for-local-ephemeral-storage).
If that is filled up from another source (for example, log files or image overlays),
the