---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#1-standard
chunk_level: standard
chunk_type: table
heading: `CommandDefaults`
token_count: 382
summary: ## `AliasOverride` **Appears in:** * [Preference](#kubectl-config-k8s-io-v1alpha1-Preference) AliasOverride stores the alias definitions. |Field|Description| |`name`**[Required]** `string`| name is...
---

## `AliasOverride`
**Appears in:**
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
prependArgs stores the arguments such as resource names, etc.
These arguments are inserted after the alias name.
|
|`appendArgs`**[Required]**
`[]string`|
appendArgs stores the arguments such as resource names, etc.
These arguments are appended to the USER\_ARGS.
|
|`flags`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1alpha1-CommandOptionDefault)|
flags is allocated to store the flag definitions of alias.
flags only modifies the default value of the flag and if
user explicitly passes a value, explicit one is used.
|
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