---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-service-account-v1.md/docs-reference-kubernetes-api-authentication-resources-service-account-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: ServiceAccountList
token_count: 280
summary: ## ServiceAccountList ServiceAccountList is a list of ServiceAccount objects * **apiVersion**: v1 * **kind**: ServiceAccountList * **metadata**...
---

## ServiceAccountList
ServiceAccountList is a list of ServiceAccount objects
* **apiVersion**: v1
* **kind**: ServiceAccountList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **items** ([][ServiceAccount](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/#ServiceAccount)), required
List of ServiceAccounts. More info: [https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)
#### Parameters
* **name** (*in path*): string, required
name of the ServiceAccount
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)