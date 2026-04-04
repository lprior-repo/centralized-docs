---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#48-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 120
summary: * **features** (NodeFeatures) Features describes the set of features implemented by the CRI implementation. *NodeFeatures describes the set of features implemented by the CRI implementation. The...
---

* **features** (NodeFeatures)
Features describes the set of features implemented by the CRI implementation.
*NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.*
* **features.supplementalGroupsPolicy** (boolean)
SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
* **images** ([]ContainerImage)
*Atomic: will be replaced during a merge*
List of container images on this node
*Describe a container image*