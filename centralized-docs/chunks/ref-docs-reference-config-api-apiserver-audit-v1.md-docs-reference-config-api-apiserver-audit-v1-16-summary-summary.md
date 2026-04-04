---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#16-summary
chunk_level: summary
chunk_type: table
heading: `Policy`
token_count: 128
summary: * [PolicyList](#audit-k8s-io-v1-PolicyList) Policy defines the configuration of audit logging, and the rules for how different request categories are logged. |Field|Description| |`apiVersion`...
---

* [PolicyList](#audit-k8s-io-v1-PolicyList)
Policy defines the configuration of audit logging, and the rules for how different request
categories are logged.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`Policy`|
|`metadata`
[`meta/v1.ObjectMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#objectmeta-v1-meta)|
ObjectMeta is included for interoperability with API infrastructure.
Refer to the Kubernetes API documentation for the fields of the `metadata` field.|