---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#5-standard
chunk_level: standard
chunk_type: prose
heading: NodeSpec
token_count: 174
summary: * **taints.key** (string), required Required. The taint key to be applied to a node. * **taints.timeAdded** (Time) TimeAdded represents the time at which the taint was added. *Time is a wrapper...
---

* **taints.key** (string), required
Required. The taint key to be applied to a node.
* **taints.timeAdded** (Time)
TimeAdded represents the time at which the taint was added.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **taints.value** (string)
The taint value corresponding to the taint key.
* **unschedulable** (boolean)
Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: [https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration](https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration)