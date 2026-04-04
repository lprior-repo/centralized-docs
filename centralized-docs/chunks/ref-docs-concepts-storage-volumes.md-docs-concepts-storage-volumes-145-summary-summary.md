---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#145-summary
chunk_level: summary
chunk_type: prose
heading: Read-only mounts
token_count: 116
summary: * `Disabled` (default): no effect. * `Enabled`: makes the mount recursively read-only. Needs all the following requirements to be satisfied: * `readOnly` is set to `true` * `mountPropagation` is...
---

* `Disabled` (default): no effect.
* `Enabled`: makes the mount recursively read-only.
Needs all the following requirements to be satisfied:
* `readOnly` is set to `true`
* `mountPropagation` is unset, or set to `None`
* The host is running with Linux kernel v5.12 or later
* The [CRI-level](/docs/concepts/architecture/cri) container runtime supports recursive read-only mounts
* The OCI-level container runtime supports recursive read-only mounts.
It will fail if any of these is not true.