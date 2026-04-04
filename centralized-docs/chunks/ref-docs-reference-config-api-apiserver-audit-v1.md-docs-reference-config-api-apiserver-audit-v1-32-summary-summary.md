---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#32-summary
chunk_level: summary
chunk_type: prose
heading: `PolicyRule`
token_count: 127
summary: An empty list implies every user. | |`userGroups` `[]string`| The user groups this rule applies to. A user is considered matching if it is a member of any of the UserGroups. An empty list implies...
---

An empty list implies every user.
|
|`userGroups`
`[]string`|
The user groups this rule applies to. A user is considered matching
if it is a member of any of the UserGroups.
An empty list implies every user group.
|
|`verbs`
`[]string`|
The verbs that match this rule.
An empty list implies every verb.
|
|`resources`
[`[]GroupResources`](#audit-k8s-io-v1-GroupResources)|
Resources that this rule matches. An empty list implies all kinds in all API groups.
|
|`namespaces`
`[]string`|