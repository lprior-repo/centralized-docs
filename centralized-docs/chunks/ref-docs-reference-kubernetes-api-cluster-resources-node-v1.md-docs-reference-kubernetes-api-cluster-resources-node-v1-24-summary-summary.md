---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#24-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 71
summary: * **addresses.type** (string), required Node address type, one of Hostname, ExternalIP or InternalIP. * **allocatable**...
---

* **addresses.type** (string), required
Node address type, one of Hostname, ExternalIP or InternalIP.
* **allocatable** (map[string][Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.