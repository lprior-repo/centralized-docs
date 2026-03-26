---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#0-detailed
chunk_level: detailed
chunk_type: table
heading: `CommandOptionDefault`
token_count: 1021
summary: ## Table of Contents    - [Resource Types](#resource-types)   - [`AliasOverride`](#aliasoverride)   - [`CommandDefaults`](#commanddefaults)   - [`CommandOptionDefault`](#commandoptiondefault)   -...
---

## Table of Contents

  - [Resource Types](#resource-types)
  - [`AliasOverride`](#aliasoverride)
  - [`CommandDefaults`](#commanddefaults)
  - [`CommandOptionDefault`](#commandoptiondefault)
  - [Feedback](#feedback)

---

## Resource Types
* [Preference](#kubectl-config-k8s-io-v1alpha1-Preference)## `Preference`
Preference stores elements of KubeRC configuration file
|Field|Description|
|`apiVersion`
string|`kubectl.config.k8s.io/v1alpha1`|
|`kind`
string|`Preference`|
|`overrides`**[Required]**
[`[]CommandDefaults`](#kubectl-config-k8s-io-v1alpha1-CommandDefaults)|
overrides allows changing default flag values of commands.
This is especially useful, when user doesn't want to explicitly
set flags each time.
|
|`aliases`**[Required]**
[`[]AliasOverride`](#kubectl-config-k8s-io-v1alpha1-AliasOverride)|
aliases allow defining command aliases for existing kubectl commands, with optional default flag values.
If the alias name collides with a built-in command, built-in command always takes precedence.
Flag overrides defined in the overrides section do NOT apply to aliases for the same command.
kubectl [ALIAS NAME] [USER\_FLAGS] [USER\_EXPLICIT\_ARGS] expands to
kubectl [COMMAND] # built-in command alias points to
[KUBERC\_PREPEND\_ARGS]
[USER\_FLAGS]
[KUBERC\_FLAGS] # rest of the flags that are not passed by user in [USER\_FLAGS]
[USER\_EXPLICIT\_ARGS]
[KUBERC\_APPEND\_ARGS]
e.g.
* name: runx
command: run
flags:
* name: image
default: nginx
appendArgs:
* custom-arg1
For example, if user invokes "kubectl runx test-pod" command,
this will be expanded to "kubectl run --image=nginx test-pod -- custom-arg1"
* name: getn
command: get
flags:
* name: output
default: wide
prependArgs:
* node
"kubectl getn control-plane-1" expands to "kubectl get node control-plane-1 --output=wide"
"kubectl getn control-plane-1 --output=json" expands to "kubectl get node --output=json control-plane-1"|
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
## `CommandOptionDefault`
**Appears in:**
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