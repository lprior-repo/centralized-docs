---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-node-v1.md/docs-reference-kubernetes-api-cluster-resources-node-v1#49-summary
chunk_level: summary
chunk_type: prose
heading: NodeStatus
token_count: 80
summary: * **images.names** ([]string) *Atomic: will be replaced during a merge* Names by which this image is known. e.g. [\"kubernetes.example/hyperkube:v1.0.7\",...
---

* **images.names** ([]string)
*Atomic: will be replaced during a merge*
Names by which this image is known. e.g. ["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"]
* **images.sizeBytes** (int64)
The size of the image in bytes.