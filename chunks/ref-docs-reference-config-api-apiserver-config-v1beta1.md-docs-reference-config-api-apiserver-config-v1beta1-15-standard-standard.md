---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#15-standard
chunk_level: standard
chunk_type: table
heading: `PrefixedClaimOrExpression`
token_count: 507
summary: ## `JWTAuthenticator` **Appears in:** * [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration) JWTAuthenticator provides the configuration for a single JWT...
---

## `JWTAuthenticator`
**Appears in:**
* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
JWTAuthenticator provides the configuration for a single JWT authenticator.
|Field|Description|
|`issuer`**[Required]**
[`Issuer`](#apiserver-k8s-io-v1beta1-Issuer)|
issuer contains the basic OIDC provider connection options.
|
|`claimValidationRules`
[`[]ClaimValidationRule`](#apiserver-k8s-io-v1beta1-ClaimValidationRule)|
claimValidationRules are rules that are applied to validate token claims to authenticate users.
|
|`claimMappings`**[Required]**
[`ClaimMappings`](#apiserver-k8s-io-v1beta1-ClaimMappings)|
claimMappings points claims of a token to be treated as user attributes.
|
|`userValidationRules`
[`[]UserValidationRule`](#apiserver-k8s-io-v1beta1-UserValidationRule)|
userValidationRules are rules that are applied to final user before completing authentication.
These allow invariants to be applied to incoming identities such as preventing the
use of the system: prefix that is commonly used by Kubernetes components.
The validation rules are logically ANDed together and must all return true for the validation to pass.
|
## `PrefixedClaimOrExpression`
**Appears in:**
* [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings)
PrefixedClaimOrExpression provides the configuration for a single prefixed claim or expression.
|Field|Description|
|`claim`
`string`|
claim is the JWT claim to use.
Mutually exclusive with expression.
|
|`prefix`
`string`|
prefix is prepended to claim's value to prevent clashes with existing names.
prefix needs to be set if claim is set and can be the empty string.
Mutually exclusive with expression.
|
|`expression`
`string`|
expression represents the expression which will be evaluated by CEL.
CEL expressions have access to the contents of the token claims, organized into CEL variable:
* 'claims' is a map of claim names to claim values.
For example, a variable named 'sub' can be accessed as 'claims.sub'.
Nested claims can be accessed using dot notation, e.g. 'claims.foo.bar'.
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
Mutually exclusive with claim and prefix.
|