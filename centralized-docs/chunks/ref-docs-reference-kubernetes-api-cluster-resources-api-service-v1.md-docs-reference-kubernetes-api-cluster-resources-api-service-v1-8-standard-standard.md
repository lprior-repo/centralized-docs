---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#8-standard
chunk_level: standard
chunk_type: prose
heading: APIServiceList
token_count: 308
summary: #### Response 200 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): OK 201...
---

#### Response
200 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): OK
201 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): Created
401: Unauthorized
#### Parameters
* **name** (*in path*): string, required
name of the APIService
* **body**: [APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): OK
201 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): Created
401: Unauthorized