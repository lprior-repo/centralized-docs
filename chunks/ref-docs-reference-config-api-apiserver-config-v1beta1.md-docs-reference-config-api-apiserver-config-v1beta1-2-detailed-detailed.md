---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#2-detailed
chunk_level: detailed
chunk_type: table
heading: `AuthorizerConfiguration`
token_count: 572
summary: ## `TracingConfiguration` TracingConfiguration provides versioned configuration for tracing clients. |Field|Description| |`apiVersion` string|`apiserver.k8s.io/v1beta1`| |`kind`...
---

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
## `AuthorizerConfiguration`
**Appears in:**
* [AuthorizationConfiguration](#apiserver-k8s-io-v1beta1-AuthorizationConfiguration)|Field|Description|
|`type`**[Required]**
`string`|
Type refers to the type of the authorizer
"Webhook" is supported in the generic API server
Other API servers may support additional authorizer
types like Node, RBAC, ABAC, etc.
|
|`name`**[Required]**
`string`|
Name used to describe the webhook
This is explicitly used in monitoring machinery for metrics
Note: Names must be DNS1123 labels like `myauthorizername` or
subdomains like `myauthorizer.example.domain`
Required, with no default
|
|`webhook`**[Required]**
[`WebhookConfiguration`](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|
Webhook defines the configuration for a Webhook authorizer
Must be defined when Type=Webhook
Must not be defined when Type!=Webhook
|