---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#28-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 79
summary: * **conditions.message** (string) Human readable message indicating details about last transition. * **conditions.reason** (string) (brief) reason for the condition's last transition. * **config**...
---

* **conditions.message** (string)
Human readable message indicating details about last transition.
* **conditions.reason** (string)
(brief) reason for the condition's last transition.
* **config** (NodeConfigStatus)
Status of the config assigned to the node via the dynamic Kubelet config feature.
*NodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.*