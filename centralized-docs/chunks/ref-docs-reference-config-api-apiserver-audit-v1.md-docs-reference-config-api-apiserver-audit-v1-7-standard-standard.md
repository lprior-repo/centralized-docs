---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#7-standard
chunk_level: standard
chunk_type: table
heading: `ObjectReference`
token_count: 264
summary: ## `Level` (Alias of `string`) **Appears in:** * [Event](#audit-k8s-io-v1-Event) * [PolicyRule](#audit-k8s-io-v1-PolicyRule) Level defines the amount of information logged during auditing ##...
---

## `Level`
(Alias of `string`)
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
Level defines the amount of information logged during auditing
## `ObjectReference`
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)
ObjectReference contains enough information to let you inspect or modify the referred object.
|Field|Description|
|`resource`
`string`|No description provided.|
|`namespace`
`string`|No description provided.|
|`name`
`string`|No description provided.|
|`uid`
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|No description provided.|
|`apiGroup`
`string`|
APIGroup is the name of the API group that contains the referred object.
The empty string represents the core API group.
|
|`apiVersion`
`string`|
APIVersion is the version of the API group that contains the referred object.
|
|`resourceVersion`
`string`|No description provided.|
|`subresource`
`string`|No description provided.|