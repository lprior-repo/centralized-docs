---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#22-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 123
summary: * **addresses** ([]NodeAddress) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* List of addresses reachable to the node. Queried from cloud...
---

* **addresses** ([]NodeAddress)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
List of addresses reachable to the node. Queried from cloud provider, if available. More info: [https://kubernetes.io/docs/reference/node/node-status/#addresses](https://kubernetes.io/docs/reference/node/node-status/#addresses) Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See