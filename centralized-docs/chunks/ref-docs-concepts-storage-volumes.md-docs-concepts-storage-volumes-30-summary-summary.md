---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#30-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 68
summary: The `log-config` ConfigMap is mounted as a volume, and all contents stored in its `log\_level` entry are mounted into the Pod at path `/etc/config/log\_level.conf`. Note that this path is derived...
---

The `log-config` ConfigMap is mounted as a volume, and all contents stored in
its `log\_level` entry are mounted into the Pod at path `/etc/config/log\_level.conf`.
Note that this path is derived from the volume's `mountPath` and the `path`
keyed with `log\_level`.