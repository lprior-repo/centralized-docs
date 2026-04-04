---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#53-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 78
summary: * Access to the host filesystem can expose privileged system credentials (such as for the kubelet) or privileged APIs (such as the container runtime socket) that can be used for container escape or...
---

* Access to the host filesystem can expose privileged system credentials (such as for the kubelet) or privileged APIs
(such as the container runtime socket) that can be used for container escape or to attack other
parts of the cluster.
* Pods with identical configuration (such as created from a PodTemplate) may
behave differently on different nodes due to different files on the nodes.