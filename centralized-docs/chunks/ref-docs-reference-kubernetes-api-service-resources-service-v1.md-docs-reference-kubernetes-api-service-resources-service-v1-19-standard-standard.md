---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#19-standard
chunk_level: standard
chunk_type: prose
heading: ServiceStatus
token_count: 490
summary: #### Parameters * **namespace** (*in path*): string, required [namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace) * **body**:...
---

#### Parameters
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service)): OK
201 ([Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service)): Created
202 ([Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service)): Accepted
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the Service
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)