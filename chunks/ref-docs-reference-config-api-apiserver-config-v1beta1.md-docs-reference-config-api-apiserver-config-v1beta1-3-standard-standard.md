---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#3-standard
chunk_level: standard
chunk_type: table
heading: `AudienceMatchPolicyType`
token_count: 454
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
## `TracingConfiguration`
TracingConfiguration provides versioned configuration for tracing clients.
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`TracingConfiguration`|
|`TracingConfiguration`**[Required]**
[`TracingConfiguration`](#TracingConfiguration)|(Members of `TracingConfiguration` are embedded into this type.)
Embed the component config tracing configuration struct
|
## `AnonymousAuthCondition`
**Appears in:**
* [AnonymousAuthConfig](#apiserver-k8s-io-v1beta1-AnonymousAuthConfig)
AnonymousAuthCondition describes the condition under which anonymous auth
should be enabled.
|Field|Description|
|`path`**[Required]**
`string`|
Path for which anonymous auth is enabled.
|
## `AnonymousAuthConfig`
**Appears in:**
* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
AnonymousAuthConfig provides the configuration for the anonymous authenticator.
|Field|Description|
|`enabled`**[Required]**
`bool`|No description provided.|
|`conditions`**[Required]**
[`[]AnonymousAuthCondition`](#apiserver-k8s-io-v1beta1-AnonymousAuthCondition)|
If set, anonymous auth is only allowed if the request meets one of the
conditions.
|
## `AudienceMatchPolicyType`
(Alias of `string`)
**Appears in:**
* [Issuer](#apiserver-k8s-io-v1beta1-Issuer)
AudienceMatchPolicyType is a set of valid values for issuer.audienceMatchPolicy