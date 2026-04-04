---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#1-standard
chunk_level: standard
chunk_type: prose
heading: Workload
token_count: 210
summary: # Workload v1alpha1 Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including scheduling, preemption,...
---

# Workload v1alpha1
Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including scheduling, preemption, eviction and other phases.
`apiVersion: scheduling.k8s.io/v1alpha1`
`import "k8s.io/api/scheduling/v1alpha1"`
## Workload
Workload allows for expressing scheduling constraints that should be used when managing lifecycle of workloads from scheduling perspective, including scheduling, preemption, eviction and other phases.
* **apiVersion**: scheduling.k8s.io/v1alpha1
* **kind**: Workload
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. Name must be a DNS subdomain.
* **spec** ([WorkloadSpec](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#WorkloadSpec)), required
Spec defines the desired behavior of a Workload.