---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#45-summary
chunk_level: summary
chunk_type: table
heading: `EgressSelection`
token_count: 127
summary: * [EgressSelectorConfiguration](#apiserver-k8s-io-v1beta1-EgressSelectorConfiguration) EgressSelection provides the configuration for a single egress selection client. |Field|Description|...
---

* [EgressSelectorConfiguration](#apiserver-k8s-io-v1beta1-EgressSelectorConfiguration)
EgressSelection provides the configuration for a single egress selection client.
|Field|Description|
|`name`**[Required]**
`string`|
name is the name of the egress selection.
Currently supported values are "controlplane", "master", "etcd" and "cluster"
The "master" egress selector is deprecated in favor of "controlplane"
|
|`connection`**[Required]**
[`Connection`](#apiserver-k8s-io-v1beta1-Connection)|