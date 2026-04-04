---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#83-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 127
summary: ### local A `local` volume represents a mounted local storage device such as a disk, partition or directory. Local volumes can only be used as a statically created PersistentVolume. Dynamic...
---

### local
A `local` volume represents a mounted local storage device such as a disk,
partition or directory.
Local volumes can only be used as a statically created PersistentVolume. Dynamic
provisioning is not supported.
Compared to `hostPath` volumes, `local` volumes are used in a durable and
portable manner without manually scheduling Pods to nodes. The system is aware
of the volume's node constraints by looking at the node affinity on the PersistentVolume.
However, `local` volumes are subject to the availability of the underlying
node and are not suitable for all applications. If a node becomes unhealthy,