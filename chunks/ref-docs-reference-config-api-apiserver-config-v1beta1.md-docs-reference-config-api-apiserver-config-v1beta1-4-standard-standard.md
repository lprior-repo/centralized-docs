---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#4-standard
chunk_level: standard
chunk_type: table
heading: `AuthorizerConfiguration`
token_count: 281
summary: ## `AudienceMatchPolicyType` (Alias of `string`) **Appears in:** * [Issuer](#apiserver-k8s-io-v1beta1-Issuer) AudienceMatchPolicyType is a set of valid values for issuer.audienceMatchPolicy ##...
---

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