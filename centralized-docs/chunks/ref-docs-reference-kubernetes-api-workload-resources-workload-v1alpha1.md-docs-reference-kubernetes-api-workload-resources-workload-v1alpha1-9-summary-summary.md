---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#9-summary
chunk_level: summary
chunk_type: prose
heading: WorkloadSpec
token_count: 107
summary: * **podGroups** ([]PodGroup), required *Map: unique values on key name will be kept during a merge* PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is...
---

* **podGroups** ([]PodGroup), required
*Map: unique values on key name will be kept during a merge*
PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.
*PodGroup represents a set of pods with a common scheduling policy.*
* **podGroups.name** (string), required
Name is a unique identifier for the PodGroup within the Workload. It must be a DNS label. This field is immutable.