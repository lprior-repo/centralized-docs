---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#54-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 111
summary: * **nodeInfo.systemUUID** (string), required SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts...
---

* **nodeInfo.systemUUID** (string), required
SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts [https://access.redhat.com/documentation/en-us/red\_hat\_subscription\_management/1/html/rhsm/uuid](https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid)
* **nodeInfo.swap** (NodeSwapStatus)
Swap Info reported by the node.
*NodeSwapStatus represents swap memory information.*