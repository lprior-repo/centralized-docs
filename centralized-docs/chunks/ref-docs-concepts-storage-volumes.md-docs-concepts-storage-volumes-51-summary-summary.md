---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#51-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 123
summary: Using the `hostPath` volume type presents many security risks. If you can avoid using a `hostPath` volume, you should. For example, define a [`local` PersistentVolume](#local), and use that instead....
---

Using the `hostPath` volume type presents many security risks.
If you can avoid using a `hostPath` volume, you should. For example,
define a [`local` PersistentVolume](#local), and use that instead.
If you are restricting access to specific directories on the node using
admission-time validation, that restriction is only effective when you
additionally require that any mounts of that `hostPath` volume are
**read only**. If you allow a read-write mount of any host path by an
untrusted Pod, the containers in that Pod may be able to subvert the