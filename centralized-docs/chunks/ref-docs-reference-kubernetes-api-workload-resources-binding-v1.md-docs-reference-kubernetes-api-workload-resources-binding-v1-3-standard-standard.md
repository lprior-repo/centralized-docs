---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1#3-standard
chunk_level: standard
chunk_type: prose
heading: Binding
token_count: 381
summary: #### Response 200 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): OK 201...
---

#### Response
200 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): OK
201 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): Created
202 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): Accepted
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the Binding
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): OK
201 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): Created
202 ([Binding](https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/binding-v1/#Binding)): Accepted
401: Unauthorized