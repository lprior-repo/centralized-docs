---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#23-summary
chunk_level: summary
chunk_type: table
heading: `GroupResources`
token_count: 120
summary: * [PolicyRule](#audit-k8s-io-v1-PolicyRule) GroupResources represents resource kinds in an API group. |Field|Description| |`group` `string`| Group is the name of the API group that contains the...
---

* [PolicyRule](#audit-k8s-io-v1-PolicyRule)
GroupResources represents resource kinds in an API group.
|Field|Description|
|`group`
`string`|
Group is the name of the API group that contains the resources.
The empty string represents the core API group.
|
|`resources`
`[]string`|
Resources is a list of resources this rule applies to.
For example:
* `pods` matches pods.
* `pods/log` matches the log subresource of pods.
* `\*` matches all resources and their subresources.