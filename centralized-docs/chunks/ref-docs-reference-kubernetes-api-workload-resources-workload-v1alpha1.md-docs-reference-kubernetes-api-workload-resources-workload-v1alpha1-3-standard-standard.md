---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#3-standard
chunk_level: standard
chunk_type: prose
heading: WorkloadList
token_count: 198
summary: ## WorkloadList WorkloadList contains a list of Workload resources. * **apiVersion**: scheduling.k8s.io/v1alpha1 * **kind**: WorkloadList * **metadata**...
---

## WorkloadList
WorkloadList contains a list of Workload resources.
* **apiVersion**: scheduling.k8s.io/v1alpha1
* **kind**: WorkloadList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata.
* **items** ([][Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)), required
Items is the list of Workloads.
#### Parameters
* **name** (*in path*): string, required
name of the Workload
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)