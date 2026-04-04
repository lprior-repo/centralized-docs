---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#26-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 122
summary: * **conditions** ([]NodeCondition) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* Conditions is an array of current observed node conditions. More...
---

* **conditions** ([]NodeCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions is an array of current observed node conditions. More info: [https://kubernetes.io/docs/reference/node/node-status/#condition](https://kubernetes.io/docs/reference/node/node-status/#condition)
*NodeCondition contains condition information for a node.*
* **conditions.status** (string), required
Status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
Type of node condition.