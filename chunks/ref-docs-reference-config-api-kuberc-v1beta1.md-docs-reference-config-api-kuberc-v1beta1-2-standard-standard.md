---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#2-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 390
summary: * [Preference](#kubectl-config-k8s-io-v1beta1-Preference)## `Preference` Preference stores elements of KubeRC configuration file |Field|Description| |`apiVersion`...
---

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)## `Preference`
Preference stores elements of KubeRC configuration file
|Field|Description|
|`apiVersion`
string|`kubectl.config.k8s.io/v1beta1`|
|`kind`
string|`Preference`|
|`defaults`**[Required]**
[`[]CommandDefaults`](#kubectl-config-k8s-io-v1beta1-CommandDefaults)|
defaults allow changing default option values of commands.
This is especially useful, when user doesn't want to explicitly
set options each time.
|
|`aliases`**[Required]**
[`[]AliasOverride`](#kubectl-config-k8s-io-v1beta1-AliasOverride)|
aliases allow defining command aliases for existing kubectl commands, with optional default option values.
If the alias name collides with a built-in command, built-in command always takes precedence.
Option overrides defined in the defaults section do NOT apply to aliases for the same command.
kubectl [ALIAS NAME] [USER\_OPTIONS] [USER\_EXPLICIT\_ARGS] expands to
kubectl [COMMAND] # built-in command alias points to
[KUBERC\_PREPEND\_ARGS]
[USER\_OPTIONS]
[KUBERC\_OPTIONS] # rest of the options that are not passed by user in [USER\_OPTIONS]
[USER\_EXPLICIT\_ARGS]
[KUBERC\_APPEND\_ARGS]
e.g.
* name: runx
command: run
options:
* name: image
default: nginx
appendArgs:
* custom-arg1
For example, if user invokes "kubectl runx test-pod" command,
this will be expanded to "kubectl run --image=nginx test-pod -- custom-arg1"
* name: getn
command: get
options:
* name: output
default: wide
prependArgs: