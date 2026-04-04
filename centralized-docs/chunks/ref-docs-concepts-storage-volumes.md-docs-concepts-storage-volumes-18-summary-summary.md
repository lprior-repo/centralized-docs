---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#18-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 110
summary: * durably storing data so that it stays available even if the Pod restarts or is replaced * passing configuration information to an app running in a container, based on details of the Pod the...
---

* durably storing data so that it stays available even if the Pod restarts or is replaced
* passing configuration information to an app running in a container, based on details of the Pod
the container is in
(for example: telling a [sidecar container](/docs/concepts/workloads/pods/sidecar-containers/)
what namespace the Pod is running in)
* providing read-only access to data in a different container image
Data sharing can be between different local processes within a container, or between different containers,
or between Pods.