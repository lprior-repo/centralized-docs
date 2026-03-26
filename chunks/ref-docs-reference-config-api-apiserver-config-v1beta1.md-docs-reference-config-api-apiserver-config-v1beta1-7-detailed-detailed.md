---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#7-detailed
chunk_level: detailed
chunk_type: table
heading: `TLSConfig`
token_count: 896
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
## `TCPTransport`
**Appears in:**
* [Transport](#apiserver-k8s-io-v1beta1-Transport)
TCPTransport provides the information to connect to konnectivity server via TCP
|Field|Description|
|`url`**[Required]**
`string`|
URL is the location of the konnectivity server to connect to.
As an example it might be "https://127.0.0.1:8131"
|
|`tlsConfig`
[`TLSConfig`](#apiserver-k8s-io-v1beta1-TLSConfig)|
TLSConfig is the config needed to use TLS when connecting to konnectivity server
|
## `TLSConfig`
**Appears in:**
* [TCPTransport](#apiserver-k8s-io-v1beta1-TCPTransport)
TLSConfig provides the authentication information to connect to konnectivity server
Only used with TCPTransport
|Field|Description|
|`caBundle`
`string`|
caBundle is the file location of the CA to be used to determine trust with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
If absent while TCPTransport.URL is prefixed with https://, default to system trust roots.
|
|`clientKey`
`string`|
clientKey is the file location of the client key to be used in mtls handshakes with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
Must be configured if TCPTransport.URL is prefixed with https://
|
|`clientCert`
`string`|
clientCert is the file location of the client certificate to be used in mtls handshakes with the konnectivity server.
Must be absent/empty if TCPTransport.URL is prefixed with http://
Must be configured if TCPTransport.URL is prefixed with https://
|