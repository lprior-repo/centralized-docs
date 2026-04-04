---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#13-summary
chunk_level: summary
chunk_type: prose
heading: WorkloadList
token_count: 110
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