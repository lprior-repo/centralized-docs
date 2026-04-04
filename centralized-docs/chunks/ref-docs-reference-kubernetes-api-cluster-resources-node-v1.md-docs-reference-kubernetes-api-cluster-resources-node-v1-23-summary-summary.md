---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#23-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 128
summary: [https://pr.k8s.io/79391](https://pr.k8s.io/79391) for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may...
---

[https://pr.k8s.io/79391](https://pr.k8s.io/79391) for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
*NodeAddress contains information for the node's address.*
* **addresses.address** (string), required
The node address.
* **addresses.type** (string), required
Node address type, one of Hostname, ExternalIP or InternalIP.