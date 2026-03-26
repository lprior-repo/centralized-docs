---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#4-standard
chunk_level: standard
chunk_type: table
heading: `AllowlistEntry`
token_count: 480
summary: ## `AliasOverride` **Appears in:** * [Preference](#kubectl-config-k8s-io-v1beta1-Preference) AliasOverride stores the alias definitions. |Field|Description| |`name`**[Required]** `string`| name is...
---

## `AliasOverride`
**Appears in:**
* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
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
|`options`**[Required]**
[`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1beta1-CommandOptionDefault)|
options is allocated to store the option definitions of alias.
options only modify the default value of the option and if
user explicitly passes a value, explicit one is used.
|
## `AllowlistEntry`
**Appears in:**
* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
AllowlistEntry is an entry in the allowlist. For each allowlist item, at
least one field must be nonempty. A struct with all empty fields is
considered a misconfiguration error. Each field is a criterion for
execution. If multiple fields are specified, then the criteria of all
specified fields must be met. That is, the result of an individual entry is
the logical AND of all checks corresponding to the specified fields within
the entry.
|Field|Description|
|`name`**[Required]**
`string`|
Name matching is performed by first resolving the absolute path of both
the plugin and the name in the allowlist entry using `exec.LookPath`. It
will be called on both, and the resulting strings must be equal. If
either call to `exec.LookPath` results in an error, the `Name` check
will be considered a failure.
|