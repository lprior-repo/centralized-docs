---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#27-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 126
summary: * **conditions.type** (string), required Type of node condition. * **conditions.lastHeartbeatTime** (Time) Last time we got an update on a given condition. *Time is a wrapper around time.Time which...
---

* **conditions.type** (string), required
Type of node condition.
* **conditions.lastHeartbeatTime** (Time)
Last time we got an update on a given condition.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.lastTransitionTime** (Time)
Last time the condition transit from one status to another.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*