---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#5-standard
chunk_level: standard
chunk_type: prose
heading: APIServiceList
token_count: 268
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