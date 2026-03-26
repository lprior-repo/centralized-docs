---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#0-detailed
chunk_level: detailed
chunk_type: table
heading: Resource Types
token_count: 821
summary: ## Table of Contents    - [Resource Types](#resource-types)   - [`AliasOverride`](#aliasoverride)   - [`AllowlistEntry`](#allowlistentry)   - [`CommandDefaults`](#commanddefaults)   -...
---

## Table of Contents

  - [Resource Types](#resource-types)
  - [`AliasOverride`](#aliasoverride)
  - [`AllowlistEntry`](#allowlistentry)
  - [`CommandDefaults`](#commanddefaults)
  - [`CommandOptionDefault`](#commandoptiondefault)
  - [`CredentialPluginPolicy`](#credentialpluginpolicy)
  - [Feedback](#feedback)

---

## Resource Types
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
* node
"kubectl getn control-plane-1" expands to "kubectl get node control-plane-1 --output=wide"
"kubectl getn control-plane-1 --output=json" expands to "kubectl get node --output=json control-plane-1"|
|`credentialPluginPolicy`
[`CredentialPluginPolicy`](#kubectl-config-k8s-io-v1beta1-CredentialPluginPolicy)|
credentialPluginPolicy specifies the policy governing which, if any, client-go
credential plugins may be executed. It MUST be one of { "", "AllowAll", "DenyAll", "Allowlist" }.
If the policy is "", then it falls back to "AllowAll" (this is required
to maintain backward compatibility). If the policy is DenyAll, no
credential plugins may run. If the policy is Allowlist, only those
plugins meeting the criteria specified in the `credentialPluginAllowlist`
field may run.
|
|`credentialPluginAllowlist`
[`[]AllowlistEntry`](#kubectl-config-k8s-io-v1beta1-AllowlistEntry)|
Allowlist is a slice of allowlist entries. If any of them is a match,
then the executable in question may execute. That is, the result is the
logical OR of all entries in the allowlist. This list MUST NOT be
supplied if the policy is not "Allowlist".
e.g.
credentialPluginAllowlist:
* name: cloud-provider-plugin
* name: /usr/local/bin/my-plugin
In the above example, the user allows the credential plugins
`cloud-provider-plugin` (found somewhere in PATH), and the plugin found
at the explicit path `/usr/local/bin/my-plugin`.|