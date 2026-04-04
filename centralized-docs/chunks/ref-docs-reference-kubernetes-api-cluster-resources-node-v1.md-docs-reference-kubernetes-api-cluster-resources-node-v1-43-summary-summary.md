---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#43-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 62
summary: You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future. *NodeConfigSource specifies a source...
---

You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*