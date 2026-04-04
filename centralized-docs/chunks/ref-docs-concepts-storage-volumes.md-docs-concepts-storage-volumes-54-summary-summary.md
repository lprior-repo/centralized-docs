---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#54-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 124
summary: * Pods with identical configuration (such as created from a PodTemplate) may behave differently on different nodes due to different files on the nodes. * `hostPath` volume usage is not treated as...
---

* Pods with identical configuration (such as created from a PodTemplate) may
behave differently on different nodes due to different files on the nodes.
* `hostPath` volume usage is not treated as ephemeral storage usage.
You need to monitor the disk usage by yourself because excessive `hostPath` disk
usage will lead to disk pressure on the node.
Some uses for a `hostPath` are:
* running a container that needs access to node-level system components
(such as a container that transfers system logs to a central location,
accessing those logs using a read-only mount of `/var/log`)