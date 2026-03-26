---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#17-summary
chunk_level: summary
chunk_type: table
heading: `CommandDefaults`
token_count: 119
summary: ## `CommandDefaults` **Appears in:** * [Preference](#kubectl-config-k8s-io-v1beta1-Preference) CommandDefaults stores the commands and their associated option's default values. |Field|Description|...
---

## `CommandDefaults`
**Appears in:**
* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
CommandDefaults stores the commands and their associated option's
default values.
|Field|Description|
|`command`**[Required]**
`string`|
command refers to a command whose option's default value is changed.
|
|`options`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1beta1-CommandOptionDefault)|
options is a list of options storing different default values.
|