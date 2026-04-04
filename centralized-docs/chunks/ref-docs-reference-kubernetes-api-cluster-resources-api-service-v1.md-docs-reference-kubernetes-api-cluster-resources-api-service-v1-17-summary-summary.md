---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: APIServiceSpec
token_count: 110
summary: * **service** (ServiceReference) Service is a reference to the service for this API server. It must communicate on port 443. If the Service is nil, that means the handling for the API groupversion is...
---

* **service** (ServiceReference)
Service is a reference to the service for this API server. It must communicate on port 443. If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
*ServiceReference holds a reference to Service.legacy.k8s.io*
* **service.name** (string)
Name is the name of the service
* **service.namespace** (string)
Namespace is the namespace of the service