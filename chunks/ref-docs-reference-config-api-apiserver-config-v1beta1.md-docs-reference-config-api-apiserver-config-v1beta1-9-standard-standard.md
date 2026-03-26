---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#9-standard
chunk_level: standard
chunk_type: table
heading: `Connection`
token_count: 473
summary: ## `ClaimValidationRule` **Appears in:** * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) ClaimValidationRule provides the configuration for a single claim validation rule....
---

## `ClaimValidationRule`
**Appears in:**
* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
ClaimValidationRule provides the configuration for a single claim validation rule.
|Field|Description|
|`claim`
`string`|
claim is the name of a required claim.
Same as --oidc-required-claim flag.
Only string claim keys are supported.
Mutually exclusive with expression and message.
|
|`requiredValue`
`string`|
requiredValue is the value of a required claim.
Same as --oidc-required-claim flag.
Only string claim values are supported.
If claim is set and requiredValue is not set, the claim must be present with a value set to the empty string.
Mutually exclusive with expression and message.
|
|`expression`
`string`|
expression represents the expression which will be evaluated by CEL.
Must produce a boolean.
CEL expressions have access to the contents of the token claims, organized into CEL variable:
* 'claims' is a map of claim names to claim values.
For example, a variable named 'sub' can be accessed as 'claims.sub'.
Nested claims can be accessed using dot notation, e.g. 'claims.foo.bar'.
Must return true for the validation to pass.
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
Mutually exclusive with claim and requiredValue.
|
|`message`
`string`|
message customizes the returned error message when expression returns false.
message is a literal string.
Mutually exclusive with claim and requiredValue.
|
## `Connection`
**Appears in:**
* [EgressSelection](#apiserver-k8s-io-v1beta1-EgressSelection)
Connection provides the configuration for a single egress selection client.
|Field|Description|
|`proxyProtocol`**[Required]**
[`ProtocolType`](#apiserver-k8s-io-v1beta1-ProtocolType)|
Protocol is the protocol used to connect from client to the konnectivity server.
|
|`transport`
[`Transport`](#apiserver-k8s-io-v1beta1-Transport)|
Transport defines the transport configurations we use to dial to the konnectivity server.
This is required if ProxyProtocol is HTTPConnect or GRPC.
|