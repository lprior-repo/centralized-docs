---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#20-summary
chunk_level: summary
chunk_type: table
heading: `PolicyList`
token_count: 112
summary: ## `PolicyList` PolicyList is a list of audit Policies. |Field|Description| |`apiVersion` string|`audit.k8s.io/v1`| |`kind` string|`PolicyList`| |`metadata`...
---

## `PolicyList`
PolicyList is a list of audit Policies.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`PolicyList`|
|`metadata`
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#listmeta-v1-meta)|No description provided.|
|`items`**[Required]**
[`[]Policy`](#audit-k8s-io-v1-Policy)|No description provided.|