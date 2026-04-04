---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#24-summary
chunk_level: summary
chunk_type: prose
heading: How volumes work
token_count: 124
summary: however, Kubernetes does not destroy persistent volumes. For any kind of volume in a given Pod, data is preserved across container restarts. At its core, a volume is a directory, possibly with some...
---

however, Kubernetes does not destroy persistent volumes.
For any kind of volume in a given Pod, data is preserved across container restarts.
At its core, a volume is a directory, possibly with some data in it, which
is accessible to the containers in a pod. How that directory comes to be, the
medium that backs it, and the contents of it are determined by the particular
volume type used.
To use a volume, specify the volumes to provide for the Pod in `.spec.volumes`
and declare where to mount those volumes into containers in `.spec.containers[\*].volumeMounts`.