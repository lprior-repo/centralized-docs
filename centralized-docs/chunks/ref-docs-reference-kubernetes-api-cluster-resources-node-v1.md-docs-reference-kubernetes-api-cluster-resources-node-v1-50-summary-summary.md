---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#50-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 126
summary: * **images.sizeBytes** (int64) The size of the image in bytes. * **nodeInfo** (NodeSystemInfo) Set of ids/uuids to uniquely identify the node. More info:...
---

* **images.sizeBytes** (int64)
The size of the image in bytes.
* **nodeInfo** (NodeSystemInfo)
Set of ids/uuids to uniquely identify the node. More info: [https://kubernetes.io/docs/reference/node/node-status/#info](https://kubernetes.io/docs/reference/node/node-status/#info)
*NodeSystemInfo is a set of ids/uuids to uniquely identify the node.*
* **nodeInfo.architecture** (string), required
The Architecture reported by the node
* **nodeInfo.bootID** (string), required
Boot ID reported by the node.