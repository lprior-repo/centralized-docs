---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#76-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 109
summary: backoff and will be reported on the Pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine. At a...
---

backoff and will be reported on the Pod reason and message.
The types of objects that may be mounted by this volume are defined by the
container runtime implementation on a host machine. At a minimum, they must include
all valid types supported by the container image field. The OCI object gets
mounted in a single directory (`spec.containers[\*].volumeMounts[\*].mountPath`)
and will be mounted read-only.
Besides that:
* [`subPath`](/docs/concepts/storage/volumes/#using-subpath) or