---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#1-detailed
chunk_level: detailed
chunk_type: table
heading: `CredentialPluginPolicy`
token_count: 901
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
## `CommandOptionDefault`
**Appears in:**
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
## `CredentialPluginPolicy`
(Alias of `string`)
**Appears in:**
* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
CredentialPluginPolicy specifies the policy governing which, if any, client-go
credential plugins may be executed. It MUST be one of { "", "AllowAll", "DenyAll", "Allowlist" }.
If the policy is "", then it falls back to "AllowAll" (this is required
to maintain backward compatibility). If the policy is DenyAll, no
credential plugins may run. If the policy is Allowlist, only those
plugins meeting the criteria specified in the `credentialPluginAllowlist`
field may run. If the policy is not `Allowlist` but one is provided, it
is considered a configuration error.