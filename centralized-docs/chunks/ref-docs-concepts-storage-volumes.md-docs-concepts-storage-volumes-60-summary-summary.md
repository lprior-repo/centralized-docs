---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#60-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 123
summary: The `FileOrCreate` mode does **not** create the parent directory of the file. If the parent directory of the mounted file does not exist, the Pod fails to start. To ensure that this mode works, you...
---

The `FileOrCreate` mode does **not** create the parent directory of the file. If the parent directory
of the mounted file does not exist, the Pod fails to start. To ensure that this mode works,
you can try to mount directories and files separately, as shown in the
[`FileOrCreate` example](#hostpath-fileorcreate-example) for `hostPath`.
Some files or directories created on the underlying hosts might only be
accessible by root. You then either need to run your process as root in a
[privileged container](/docs/tasks/configure-pod-container/security-context/)