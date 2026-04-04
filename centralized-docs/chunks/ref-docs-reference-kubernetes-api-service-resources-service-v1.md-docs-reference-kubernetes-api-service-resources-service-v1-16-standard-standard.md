---
doc_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1
chunk_id: ref/docs-reference-kubernetes-api-service-resources-service-v1.md/docs-reference-kubernetes-api-service-resources-service-v1#16-standard
chunk_level: standard
chunk_type: prose
heading: ServiceStatus
token_count: 340
summary: * cloud provider specific error values must have names that comply with the format foo.example.com/CamelCase.## ServiceList ServiceList holds a list of services. * **apiVersion**: v1 * **kind**:...
---

* cloud provider specific error values must have names that comply with the
format foo.example.com/CamelCase.## ServiceList
ServiceList holds a list of services.
* **apiVersion**: v1
* **kind**: ServiceList
* **metadata** ([ListMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/list-meta/#ListMeta))
Standard list metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **items** ([][Service](https://kubernetes.io/docs/reference/kubernetes-api/service-resources/service-v1/#Service)), required
List of services
#### Parameters
* **name** (*in path*): string, required
name of the Service
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Parameters
* **name** (*in path*): string, required
name of the Service
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)