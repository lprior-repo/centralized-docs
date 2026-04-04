---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#3-detailed
chunk_level: detailed
chunk_type: prose
heading: APIServiceList
token_count: 909
summary: ## APIServiceList APIServiceList is a list of APIService objects. * **apiVersion**: apiregistration.k8s.io/v1 * **kind**: APIServiceList * **metadata**...
---

## APIServiceList
APIServiceList is a list of APIService objects.
* **apiVersion**: apiregistration.k8s.io/v1
* **kind**: APIServiceList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **items** ([][APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)), required
Items is the list of APIService
#### Parameters
* **name** (*in path*): string, required
name of the APIService
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the APIService
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **allowWatchBookmarks** (*in query*): boolean
[allowWatchBookmarks](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#allowWatchBookmarks)
* **continue** (*in query*): string
[continue](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#continue)
* **fieldSelector** (*in query*): string
[fieldSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldSelector)
* **labelSelector** (*in query*): string
[labelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#labelSelector)
* **limit** (*in query*): integer
[limit](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#limit)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
* **resourceVersion** (*in query*): string
[resourceVersion](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersion)
* **resourceVersionMatch** (*in query*): string
[resourceVersionMatch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#resourceVersionMatch)
* **sendInitialEvents** (*in query*): boolean
[sendInitialEvents](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#sendInitialEvents)
* **timeoutSeconds** (*in query*): integer
[timeoutSeconds](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#timeoutSeconds)
* **watch** (*in query*): boolean
[watch](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#watch)
#### Parameters
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
202 ([APIService](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/api-service-v1/#APIService)): Accepted
401: Unauthorized