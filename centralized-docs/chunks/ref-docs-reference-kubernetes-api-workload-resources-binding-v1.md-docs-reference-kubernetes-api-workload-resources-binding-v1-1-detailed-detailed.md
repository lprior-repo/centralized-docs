---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-binding-v1.md/docs-reference-kubernetes-api-workload-resources-binding-v1#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Feedback
token_count: 1023
summary: # Binding Binding ties one object to another; for example, a pod is bound to a node by a scheduler. `apiVersion: v1` `import \"k8s.io/api/core/v1\"` ## Binding Binding ties one object to another; for...
---

# Binding
Binding ties one object to another; for example, a pod is bound to a node by a scheduler.
`apiVersion: v1`
`import "k8s.io/api/core/v1"`
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified April 09, 2025 at 6:36 PM PST: [Update API reference docs for v1.32 (a3b579d035)](https://github.com/kubernetes/website/commit/a3b579d03512e440250c5153dacf982b9a364d2c)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.