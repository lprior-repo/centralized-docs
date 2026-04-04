---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: NodeSpec
token_count: 98
summary: * **podCIDR** (string) PodCIDR represents the pod IP range assigned to the node. * **podCIDRs** ([]string) *Set: unique values will be kept during a merge* podCIDRs represents the IP ranges assigned...
---

* **podCIDR** (string)
PodCIDR represents the pod IP range assigned to the node.
* **podCIDRs** ([]string)
*Set: unique values will be kept during a merge*
podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.