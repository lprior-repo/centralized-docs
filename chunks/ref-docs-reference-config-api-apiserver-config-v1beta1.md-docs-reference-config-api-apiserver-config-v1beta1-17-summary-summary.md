---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#17-summary
chunk_level: summary
chunk_type: table
heading: `EgressSelectorConfiguration`
token_count: 106
summary: ## `EgressSelectorConfiguration` EgressSelectorConfiguration provides versioned configuration for egress selector clients. |Field|Description| |`apiVersion` string|`apiserver.k8s.io/v1beta1`| |`kind`...
---

## `EgressSelectorConfiguration`
EgressSelectorConfiguration provides versioned configuration for egress selector clients.
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`EgressSelectorConfiguration`|
|`egressSelections`**[Required]**
[`[]EgressSelection`](#apiserver-k8s-io-v1beta1-EgressSelection)|
connectionServices contains a list of egress selection client configurations
|