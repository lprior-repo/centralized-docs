---
doc_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1
chunk_id: ref/docs-reference-config-api-kuberc-v1alpha1.md/docs-reference-config-api-kuberc-v1alpha1#0-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 503
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