---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#3-standard
chunk_level: standard
chunk_type: prose
heading: Resource Types
token_count: 438
summary: * name: runx command: run options: * name: image default: nginx appendArgs: * custom-arg1 For example, if user invokes \"kubectl runx test-pod\" command, this will be expanded to \"kubectl run...
---

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