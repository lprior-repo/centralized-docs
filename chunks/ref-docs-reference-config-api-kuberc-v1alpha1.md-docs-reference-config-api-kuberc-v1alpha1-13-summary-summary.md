---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#13-summary
chunk_level: summary
chunk_type: table
heading: `CommandOptionDefault`
token_count: 123
summary: * [AliasOverride](#kubectl-config-k8s-io-v1alpha1-AliasOverride) * [CommandDefaults](#kubectl-config-k8s-io-v1alpha1-CommandDefaults) CommandOptionDefault stores the name and the specified default...
---

* [AliasOverride](#kubectl-config-k8s-io-v1alpha1-AliasOverride)
* [CommandDefaults](#kubectl-config-k8s-io-v1alpha1-CommandDefaults)
CommandOptionDefault stores the name and the specified default
value of an option.
|Field|Description|
|`name`**[Required]**
`string`|
Flag name (long form, without dashes).
|
|`default`**[Required]**
`string`|
In a string format of a default value. It will be parsed
by kubectl to the compatible value of the flag.
|