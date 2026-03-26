---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#8-summary
chunk_level: summary
chunk_type: table
heading: `AliasOverride`
token_count: 125
summary: * [Preference](#kubectl-config-k8s-io-v1alpha1-Preference) AliasOverride stores the alias definitions. |Field|Description| |`name`**[Required]** `string`| name is the name of alias that can only...
---

* [Preference](#kubectl-config-k8s-io-v1alpha1-Preference)
AliasOverride stores the alias definitions.
|Field|Description|
|`name`**[Required]**
`string`|
name is the name of alias that can only include alphabetical characters
If the alias name conflicts with the built-in command,
built-in command will be used.
|
|`command`**[Required]**
`string`|
command is the single or set of commands to execute, such as "set env" or "create"
|
|`prependArgs`**[Required]**
`[]string`|