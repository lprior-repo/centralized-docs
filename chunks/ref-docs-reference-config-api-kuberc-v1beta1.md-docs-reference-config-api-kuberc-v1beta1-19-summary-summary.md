---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#19-summary
chunk_level: summary
chunk_type: table
heading: `CommandOptionDefault`
token_count: 123
summary: * [AliasOverride](#kubectl-config-k8s-io-v1beta1-AliasOverride) * [CommandDefaults](#kubectl-config-k8s-io-v1beta1-CommandDefaults) CommandOptionDefault stores the name and the specified default...
---

* [AliasOverride](#kubectl-config-k8s-io-v1beta1-AliasOverride)
* [CommandDefaults](#kubectl-config-k8s-io-v1beta1-CommandDefaults)
CommandOptionDefault stores the name and the specified default
value of an option.
|Field|Description|
|`name`**[Required]**
`string`|
Option name (long form, without dashes).
|
|`default`**[Required]**
`string`|
In a string format of a default value. It will be parsed
by kubectl to the compatible value of the option.
|