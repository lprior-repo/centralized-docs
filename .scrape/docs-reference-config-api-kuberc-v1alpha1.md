---
url: https://kubernetes.io/docs/reference/config-api/kuberc.v1alpha1/
title: kuberc.v1alpha1
word_count: 523
filtered: true
elements_removed: 0
density_score: 0.85
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified September 04, 2025 at 5:02 PM PST: [Config API for v1.34 (3557e3070d)](https://github.com/kubernetes/website/commit/3557e3070dcd5659f259e302b53f98adfd9a79f1)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)
