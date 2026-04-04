---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#12-summary
chunk_level: summary
chunk_type: prose
heading: WorkloadSpec
token_count: 115
summary: * **controllerRef.kind** (string), required Kind is the type of resource being referenced. It must be a path segment name. * **controllerRef.name** (string), required Name is the name of resource...
---

* **controllerRef.kind** (string), required
Kind is the type of resource being referenced. It must be a path segment name.
* **controllerRef.name** (string), required
Name is the name of resource being referenced. It must be a path segment name.
* **controllerRef.apiGroup** (string)
APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.