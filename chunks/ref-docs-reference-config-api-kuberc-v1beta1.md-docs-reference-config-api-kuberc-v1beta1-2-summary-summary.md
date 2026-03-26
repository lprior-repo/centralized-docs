---
doc_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1
chunk_id: ref/docs-reference-config-api-kuberc-v1beta1.md/docs-reference-config-api-kuberc-v1beta1#2-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 126
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
|