---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#29-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 90
summary: * **config.active** (NodeConfigSource) Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current...
---

* **config.active** (NodeConfigSource)
Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
*NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22*