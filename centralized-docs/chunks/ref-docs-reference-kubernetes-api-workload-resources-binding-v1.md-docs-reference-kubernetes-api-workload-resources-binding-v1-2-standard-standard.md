---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1#2-standard
chunk_level: standard
chunk_type: prose
heading: Binding
token_count: 443
summary: ## Binding Binding ties one object to another; for example, a pod is bound to a node by a scheduler. * **apiVersion**: v1 * **kind**: Binding * **metadata**...
---

## Binding
Binding ties one object to another; for example, a pod is bound to a node by a scheduler.
* **apiVersion**: v1
* **kind**: Binding
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **target** ([ObjectReference](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-reference/#ObjectReference)), required
The target object that you want to bind to the standard object.
#### Parameters
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