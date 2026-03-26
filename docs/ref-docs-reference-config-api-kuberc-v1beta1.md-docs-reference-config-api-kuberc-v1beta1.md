---
id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
title: Docs Reference Config Api Kuberc V1beta1
category: ref
tags: ["aliasoverride", "contents", "ref", "resource", "table"]
---

# Docs Reference Config Api Kuberc V1beta1



 > 
 > **Context**: Appears in:



## Table of Contents

* [Resource Types](#resource-types)
* [`AliasOverride`](#aliasoverride)
* [`AllowlistEntry`](#allowlistentry)
* [`CommandDefaults`](#commanddefaults)
* [`CommandOptionDefault`](#commandoptiondefault)
* [`CredentialPluginPolicy`](#credentialpluginpolicy)
* [Feedback](#feedback)

---

## Resource Types

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)\## `Preference`
  Preference stores elements of KubeRC configuration file
  \|Field|Description|
  \|`apiVersion`
  string|`kubectl.config.k8s.io/v1beta1`\|
  \|`kind`
  string|`Preference`\|
  \|`defaults`**\[Required\]**
  [`[]CommandDefaults`](#kubectl-config-k8s-io-v1beta1-CommandDefaults)\|
  defaults allow changing default option values of commands.
  This is especially useful, when user doesn’t want to explicitly
  set options each time.
  \|
  \|`aliases`**\[Required\]**
  [`[]AliasOverride`](#kubectl-config-k8s-io-v1beta1-AliasOverride)\|
  aliases allow defining command aliases for existing kubectl commands, with optional default option values.
  If the alias name collides with a built-in command, built-in command always takes precedence.
  Option overrides defined in the defaults section do NOT apply to aliases for the same command.
  kubectl \[ALIAS NAME\] \[USER\_OPTIONS\] \[USER\_EXPLICIT\_ARGS\] expands to
  kubectl \[COMMAND\] # built-in command alias points to
  \[KUBERC\_PREPEND\_ARGS\]
  \[USER\_OPTIONS\]
  \[KUBERC\_OPTIONS\] # rest of the options that are not passed by user in \[USER\_OPTIONS\]
  \[USER\_EXPLICIT\_ARGS\]
  \[KUBERC\_APPEND\_ARGS\]
  e.g.
* name: runx
  command: run
  options:
* name: image
  default: nginx
  appendArgs:
* custom-arg1
  For example, if user invokes “kubectl runx test-pod” command,
  this will be expanded to “kubectl run –image=nginx test-pod – custom-arg1”
* name: getn
  command: get
  options:
* name: output
  default: wide
  prependArgs:
* node
  “kubectl getn control-plane-1” expands to “kubectl get node control-plane-1 –output=wide”
  “kubectl getn control-plane-1 –output=json” expands to “kubectl get node –output=json control-plane-1”\|
  \|`credentialPluginPolicy`
  [`CredentialPluginPolicy`](#kubectl-config-k8s-io-v1beta1-CredentialPluginPolicy)\|
  credentialPluginPolicy specifies the policy governing which, if any, client-go
  credential plugins may be executed. It MUST be one of { “”, “AllowAll”, “DenyAll”, “Allowlist” }.
  If the policy is “”, then it falls back to “AllowAll” (this is required
  to maintain backward compatibility). If the policy is DenyAll, no
  credential plugins may run. If the policy is Allowlist, only those
  plugins meeting the criteria specified in the `credentialPluginAllowlist`
  field may run.
  \|
  \|`credentialPluginAllowlist`
  [`[]AllowlistEntry`](#kubectl-config-k8s-io-v1beta1-AllowlistEntry)\|
  Allowlist is a slice of allowlist entries. If any of them is a match,
  then the executable in question may execute. That is, the result is the
  logical OR of all entries in the allowlist. This list MUST NOT be
  supplied if the policy is not “Allowlist”.
  e.g.
  credentialPluginAllowlist:
* name: cloud-provider-plugin
* name: /usr/local/bin/my-plugin
  In the above example, the user allows the credential plugins
  `cloud-provider-plugin` (found somewhere in PATH), and the plugin found
  at the explicit path `/usr/local/bin/my-plugin`.|

## `AliasOverride`

**Appears in:**

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
  AliasOverride stores the alias definitions.
  \|Field|Description|
  \|`name`**\[Required\]**
  `string`\|
  name is the name of alias that can only include alphabetical characters
  If the alias name conflicts with the built-in command,
  built-in command will be used.
  \|
  \|`command`**\[Required\]**
  `string`\|
  command is the single or set of commands to execute, such as “set env” or “create”
  \|
  \|`prependArgs`**\[Required\]**
  `[]string`\|
  prependArgs stores the arguments such as resource names, etc.
  These arguments are inserted after the alias name.
  \|
  \|`appendArgs`**\[Required\]**
  `[]string`\|
  appendArgs stores the arguments such as resource names, etc.
  These arguments are appended to the USER\_ARGS.
  \|
  \|`options`**\[Required\]**
  [`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1beta1-CommandOptionDefault)\|
  options is allocated to store the option definitions of alias.
  options only modify the default value of the option and if
  user explicitly passes a value, explicit one is used.
  \|

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
  \|Field|Description|
  \|`name`**\[Required\]**
  `string`\|
  Name matching is performed by first resolving the absolute path of both
  the plugin and the name in the allowlist entry using `exec.LookPath`. It
  will be called on both, and the resulting strings must be equal. If
  either call to `exec.LookPath` results in an error, the `Name` check
  will be considered a failure.
  \|

## `CommandDefaults`

**Appears in:**

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
  CommandDefaults stores the commands and their associated option’s
  default values.
  \|Field|Description|
  \|`command`**\[Required\]**
  `string`\|
  command refers to a command whose option’s default value is changed.
  \|
  \|`options`**\[Required\]**
  [`[]CommandOptionDefault`](#kubectl-config-k8s-io-v1beta1-CommandOptionDefault)\|
  options is a list of options storing different default values.
  \|

## `CommandOptionDefault`

**Appears in:**

* [AliasOverride](#kubectl-config-k8s-io-v1beta1-AliasOverride)
* [CommandDefaults](#kubectl-config-k8s-io-v1beta1-CommandDefaults)
  CommandOptionDefault stores the name and the specified default
  value of an option.
  \|Field|Description|
  \|`name`**\[Required\]**
  `string`\|
  Option name (long form, without dashes).
  \|
  \|`default`**\[Required\]**
  `string`\|
  In a string format of a default value. It will be parsed
  by kubectl to the compatible value of the option.
  \|

## `CredentialPluginPolicy`

(Alias of `string`)
**Appears in:**

* [Preference](#kubectl-config-k8s-io-v1beta1-Preference)
  CredentialPluginPolicy specifies the policy governing which, if any, client-go
  credential plugins may be executed. It MUST be one of { “”, “AllowAll”, “DenyAll”, “Allowlist” }.
  If the policy is “”, then it falls back to “AllowAll” (this is required
  to maintain backward compatibility). If the policy is DenyAll, no
  credential plugins may run. If the policy is Allowlist, only those
  plugins meeting the criteria specified in the `credentialPluginAllowlist`
  field may run. If the policy is not `Allowlist` but one is provided, it
  is considered a configuration error.

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
Last modified December 21, 2025 at 8:51 PM PST: [Update config API reference for v1.35 release (efb02468c4)](https://github.com/kubernetes/website/commit/efb02468c4c6e41029c1ef861f827265c365b52c)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.

## Related Pages

* [Adding entries to Pod /etc/hosts with HostAliases](./ref-docs-tasks-network-customize-hosts-file-for-pods.md-docs-tasks-network-customize-hosts-file-for-pods.md)
* [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](./tutorial-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
* [Example: Deploying Cassandra with a StatefulSet](./tutorial-docs-tutorials-stateful-application-cassandra.md-docs-tutorials-stateful-application-cassandra.md)
* [Configure Quality of Service for Pods](./tutorial-docs-tasks-configure-pod-container-quality-service-pod.md-docs-tasks-configure-pod-container-quality-service-pod.md)
* [Configure Certificate Rotation for the Kubelet](./tutorial-docs-tasks-tls-certificate-rotation.md-docs-tasks-tls-certificate-rotation.md)
## See Also

- [Documentation Index](./COMPASS.md)
