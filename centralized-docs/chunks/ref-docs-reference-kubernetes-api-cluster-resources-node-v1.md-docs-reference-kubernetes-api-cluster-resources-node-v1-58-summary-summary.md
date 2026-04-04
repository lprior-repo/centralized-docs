---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#58-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 78
summary: * **volumesAttached.devicePath** (string), required DevicePath represents the device path where the volume should be available * **volumesAttached.name** (string), required Name of the attached...
---

* **volumesAttached.devicePath** (string), required
DevicePath represents the device path where the volume should be available
* **volumesAttached.name** (string), required
Name of the attached volume
* **volumesInUse** ([]string)
*Atomic: will be replaced during a merge*
List of attachable volumes in use (mounted) by the node.