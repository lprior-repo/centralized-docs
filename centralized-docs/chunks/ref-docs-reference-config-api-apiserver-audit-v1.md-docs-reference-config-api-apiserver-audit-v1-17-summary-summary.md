---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: `Policy`
token_count: 121
summary: | ObjectMeta is included for interoperability with API infrastructure. Refer to the Kubernetes API documentation for the fields of the `metadata` field.| |`rules`**[Required]**...
---

|
ObjectMeta is included for interoperability with API infrastructure.
Refer to the Kubernetes API documentation for the fields of the `metadata` field.|
|`rules`**[Required]**
[`[]PolicyRule`](#audit-k8s-io-v1-PolicyRule)|
Rules specify the audit Level a request should be recorded at.
A request may match multiple rules, in which case the FIRST matching rule is used.
The default audit level is None, but can be overridden by a catch-all rule at the end of the list.
PolicyRules are strictly ordered.
|
|`omitStages`