---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#31-summary
chunk_level: summary
chunk_type: table
heading: `PolicyRule`
token_count: 121
summary: * [Policy](#audit-k8s-io-v1-Policy) PolicyRule maps requests based off metadata to an audit Level. Requests must match the rules of every field (an intersection of rules). |Field|Description|...
---

* [Policy](#audit-k8s-io-v1-Policy)
PolicyRule maps requests based off metadata to an audit Level.
Requests must match the rules of every field (an intersection of rules).
|Field|Description|
|`level`**[Required]**
[`Level`](#audit-k8s-io-v1-Level)|
The Level that requests matching this rule are recorded at.
|
|`users`
`[]string`|
The users (by authenticated user name) this rule applies to.
An empty list implies every user.
|
|`userGroups`
`[]string`|