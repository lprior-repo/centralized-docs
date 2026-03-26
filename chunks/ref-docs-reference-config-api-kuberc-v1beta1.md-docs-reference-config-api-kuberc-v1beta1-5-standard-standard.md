---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#5-standard
chunk_level: standard
chunk_type: table
heading: `CredentialPluginPolicy`
token_count: 421
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