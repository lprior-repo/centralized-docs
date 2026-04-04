---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#7-summary
chunk_level: summary
chunk_type: prose
heading: Workload
token_count: 105
summary: * **apiVersion**: scheduling.k8s.io/v1alpha1 * **kind**: Workload * **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))...
---

* **apiVersion**: scheduling.k8s.io/v1alpha1
* **kind**: Workload
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. Name must be a DNS subdomain.
* **spec** ([WorkloadSpec](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#WorkloadSpec)), required
Spec defines the desired behavior of a Workload.