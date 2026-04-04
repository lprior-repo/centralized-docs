---
doc_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1
chunk_id: ref/docs-reference-kubernetes-api-cluster-resources-api-service-v1.md/docs-reference-kubernetes-api-cluster-resources-api-service-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: APIServiceSpec
token_count: 109
summary: * **caBundle** ([]byte) *Atomic: will be replaced during a merge* CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate. If unspecified, system trust...
---

* **caBundle** ([]byte)
*Atomic: will be replaced during a merge*
CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate. If unspecified, system trust roots on the apiserver are used.
* **group** (string)
Group is the API group name this server hosts
* **insecureSkipTLSVerify** (boolean)
InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged. You should use the CABundle instead.