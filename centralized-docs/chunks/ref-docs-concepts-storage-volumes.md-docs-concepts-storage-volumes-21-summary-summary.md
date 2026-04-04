---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#21-summary
chunk_level: summary
chunk_type: prose
heading: Why volumes are important
token_count: 118
summary: * **Shared storage:** Another problem occurs when multiple containers are running in a `Pod` and need to share files. It can be challenging to set up and access a shared filesystem across all of the...
---

* **Shared storage:** Another problem occurs when multiple containers are running in a `Pod` and
need to share files. It can be challenging to set up
and access a shared filesystem across all of the containers.
The Kubernetes [volume](/docs/concepts/storage/volumes/) abstraction
can help you to solve both of these problems.
Before you learn about volumes, PersistentVolumes, and PersistentVolumeClaims, you should read up
about [Pods](/docs/concepts/workloads/pods/) and make sure that you understand how
Kubernetes uses Pods to run containers.