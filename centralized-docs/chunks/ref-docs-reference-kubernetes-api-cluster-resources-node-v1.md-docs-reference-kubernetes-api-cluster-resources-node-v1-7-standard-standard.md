---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#7-standard
chunk_level: standard
chunk_type: prose
heading: NodeStatus
token_count: 485
summary: * **addresses** ([]NodeAddress) *Patch strategy: merge on key `type`* *Map: unique values on key type will be kept during a merge* List of addresses reachable to the node. Queried from cloud...
---

* **addresses** ([]NodeAddress)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
List of addresses reachable to the node. Queried from cloud provider, if available. More info: [https://kubernetes.io/docs/reference/node/node-status/#addresses](https://kubernetes.io/docs/reference/node/node-status/#addresses) Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See [https://pr.k8s.io/79391](https://pr.k8s.io/79391) for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
*NodeAddress contains information for the node's address.*
* **addresses.address** (string), required
The node address.
* **addresses.type** (string), required
Node address type, one of Hostname, ExternalIP or InternalIP.
* **allocatable** (map[string][Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
* **capacity** (map[string][Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
Capacity represents the total resources of a node. More info: [https://kubernetes.io/docs/reference/node/node-status/#capacity](https://kubernetes.io/docs/reference/node/node-status/#capacity)
* **conditions** ([]NodeCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
Conditions is an array of current observed node conditions. More info: [https://kubernetes.io/docs/reference/node/node-status/#condition](https://kubernetes.io/docs/reference/node/node-status/#condition)
*NodeCondition contains condition information for a node.*
* **conditions.status** (string), required
Status of the condition, one of True, False, Unknown.
* **conditions.type** (string), required
Type of node condition.