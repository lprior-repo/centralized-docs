---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#11-summary
chunk_level: summary
chunk_type: prose
heading: WorkloadSpec
token_count: 126
summary: * **podGroups.policy.gang.minCount** (int32), required MinCount is the minimum number of pods that must be schedulable or scheduled at the same time for the scheduler to admit the entire group. It...
---

* **podGroups.policy.gang.minCount** (int32), required
MinCount is the minimum number of pods that must be schedulable or scheduled at the same time for the scheduler to admit the entire group. It must be a positive integer.
* **controllerRef** (TypedLocalObjectReference)
ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.
*TypedLocalObjectReference allows to reference typed object inside the same namespace.*