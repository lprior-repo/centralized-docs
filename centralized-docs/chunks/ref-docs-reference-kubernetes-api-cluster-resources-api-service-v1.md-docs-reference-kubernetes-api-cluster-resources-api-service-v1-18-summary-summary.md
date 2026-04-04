---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#18-summary
chunk_level: summary
chunk_type: prose
heading: APIServiceSpec
token_count: 85
summary: * **service.namespace** (string) Namespace is the namespace of the service * **service.port** (int32) If specified, the port on the service that hosting webhook. Default to 443 for backward...
---

* **service.namespace** (string)
Namespace is the namespace of the service
* **service.port** (int32)
If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
* **version** (string)
Version is the API version this server hosts. For example, "v1"