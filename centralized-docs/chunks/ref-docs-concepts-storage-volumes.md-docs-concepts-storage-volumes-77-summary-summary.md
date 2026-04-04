---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#77-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 117
summary: and will be mounted read-only. Besides that: * [`subPath`](/docs/concepts/storage/volumes/#using-subpath) or [`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment) mounts...
---

and will be mounted read-only.
Besides that:
* [`subPath`](/docs/concepts/storage/volumes/#using-subpath) or
[`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment)
mounts for containers (`spec.containers[\*].volumeMounts[\*].subPath`, `spec.containers[\*].volumeMounts[\*].subPathExpr`)
are only supported from Kubernetes v1.33.
* The field `spec.securityContext.fsGroupChangePolicy` has no effect on this
volume type.