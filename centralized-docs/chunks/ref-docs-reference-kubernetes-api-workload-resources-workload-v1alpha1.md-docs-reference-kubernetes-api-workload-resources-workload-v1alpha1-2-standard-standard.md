---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#2-standard
chunk_level: standard
chunk_type: prose
heading: WorkloadSpec
token_count: 493
summary: ## WorkloadSpec WorkloadSpec defines the desired state of a Workload. * **podGroups** ([]PodGroup), required *Map: unique values on key name will be kept during a merge* PodGroups is the list of pod...
---

## WorkloadSpec
WorkloadSpec defines the desired state of a Workload.
* **podGroups** ([]PodGroup), required
*Map: unique values on key name will be kept during a merge*
PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.
*PodGroup represents a set of pods with a common scheduling policy.*
* **podGroups.name** (string), required
Name is a unique identifier for the PodGroup within the Workload. It must be a DNS label. This field is immutable.
* **podGroups.policy** (PodGroupPolicy), required
Policy defines the scheduling policy for this PodGroup.
*PodGroupPolicy defines the scheduling configuration for a PodGroup.*
* **podGroups.policy.basic** (BasicSchedulingPolicy)
Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.
*BasicSchedulingPolicy indicates that standard Kubernetes scheduling behavior should be used.*
* **podGroups.policy.gang** (GangSchedulingPolicy)
Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.
*GangSchedulingPolicy defines the parameters for gang scheduling.*
* **podGroups.policy.gang.minCount** (int32), required
MinCount is the minimum number of pods that must be schedulable or scheduled at the same time for the scheduler to admit the entire group. It must be a positive integer.
* **controllerRef** (TypedLocalObjectReference)
ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.
*TypedLocalObjectReference allows to reference typed object inside the same namespace.*
* **controllerRef.kind** (string), required
Kind is the type of resource being referenced. It must be a path segment name.
* **controllerRef.name** (string), required
Name is the name of resource being referenced. It must be a path segment name.
* **controllerRef.apiGroup** (string)
APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.