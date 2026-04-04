---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#23-summary
chunk_level: summary
chunk_type: prose
heading: How volumes work
token_count: 123
summary: Kubernetes supports many types of volumes. A [Pod](/docs/concepts/workloads/pods/) can use any number of volume types simultaneously. [Ephemeral volume](/docs/concepts/storage/ephemeral-volumes/)...
---

Kubernetes supports many types of volumes. A [Pod](/docs/concepts/workloads/pods/)
can use any number of volume types simultaneously.
[Ephemeral volume](/docs/concepts/storage/ephemeral-volumes/) types have a lifetime linked to a specific Pod,
but [persistent volumes](/docs/concepts/storage/persistent-volumes/) exist beyond
the lifetime of any individual Pod. When a Pod ceases to exist, Kubernetes destroys ephemeral volumes;
however, Kubernetes does not destroy persistent volumes.
For any kind of volume in a given Pod, data is preserved across container restarts.