---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#11-summary
chunk_level: summary
chunk_type: table
heading: `CommandDefaults`
token_count: 119
summary: ## `CommandDefaults` **Appears in:** * [Preference](#kubectl-config-k8s-io-v1alpha1-Preference) CommandDefaults stores the commands and their associated option's default values. |Field|Description|...
---

## `CommandDefaults`
**Appears in:**
* [Preference](#kubectl-config-k8s-io-v1alpha1-Preference)
CommandDefaults stores the commands and their associated option's
default values.
|Field|Description|
|`command`**[Required]**
`string`|
command refers to a command whose flag's default value is changed.
|
|`flags`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1alpha1-CommandOptionDefault)|
flags is a list of flags storing different default values.
|