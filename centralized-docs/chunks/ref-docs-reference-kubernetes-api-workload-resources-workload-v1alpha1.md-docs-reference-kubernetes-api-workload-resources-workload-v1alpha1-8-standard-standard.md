---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#8-standard
chunk_level: standard
chunk_type: prose
heading: WorkloadList
token_count: 378
summary: #### Response 200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK 201...
---

#### Response
200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK
201 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the Workload
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Patch](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/patch/#Patch), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **force** (*in query*): boolean
[force](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#force)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): OK
201 ([Workload](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/workload-v1alpha1/#Workload)): Created
401: Unauthorized