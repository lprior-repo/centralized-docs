---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#55-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 112
summary: * **nodeInfo.swap** (NodeSwapStatus) Swap Info reported by the node. *NodeSwapStatus represents swap memory information.* * **nodeInfo.swap.capacity** (int64) Total amount of swap memory in bytes. *...
---

* **nodeInfo.swap** (NodeSwapStatus)
Swap Info reported by the node.
*NodeSwapStatus represents swap memory information.*
* **nodeInfo.swap.capacity** (int64)
Total amount of swap memory in bytes.
* **phase** (string)
NodePhase is the recently observed lifecycle phase of the node. More info: [https://kubernetes.io/docs/concepts/nodes/node/#phase](https://kubernetes.io/docs/concepts/nodes/node/#phase) The field is never populated, and now is deprecated.
Possible enum values: