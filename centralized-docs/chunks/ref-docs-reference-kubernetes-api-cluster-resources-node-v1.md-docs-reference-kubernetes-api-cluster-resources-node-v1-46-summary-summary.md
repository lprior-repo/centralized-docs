---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#46-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 114
summary: * **config.lastKnownGood.configMap.resourceVersion** (string) ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in...
---

* **config.lastKnownGood.configMap.resourceVersion** (string)
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **config.lastKnownGood.configMap.uid** (string)
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
* **daemonEndpoints** (NodeDaemonEndpoints)
Endpoints of daemons running on the Node.
*NodeDaemonEndpoints lists ports opened by daemons running on the Node.*