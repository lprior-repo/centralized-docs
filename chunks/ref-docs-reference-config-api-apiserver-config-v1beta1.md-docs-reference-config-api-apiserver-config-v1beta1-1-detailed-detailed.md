---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#1-detailed
chunk_level: detailed
chunk_type: table
heading: `TracingConfiguration`
token_count: 953
summary: # kube-apiserver Configuration (v1beta1) Package v1beta1 is the v1beta1 version of the API. ## Resource Types * [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration) *...
---

# kube-apiserver Configuration (v1beta1)
Package v1beta1 is the v1beta1 version of the API.
## Resource Types
* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
* [AuthorizationConfiguration](#apiserver-k8s-io-v1beta1-AuthorizationConfiguration)
* [EgressSelectorConfiguration](#apiserver-k8s-io-v1beta1-EgressSelectorConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1beta1-TracingConfiguration)## `TracingConfiguration`
**Appears in:**
* [KubeletConfiguration](#kubelet-config-k8s-io-v1beta1-KubeletConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1alpha1-TracingConfiguration)
* [TracingConfiguration](#apiserver-k8s-io-v1beta1-TracingConfiguration)
TracingConfiguration provides versioned configuration for OpenTelemetry tracing clients.
|Field|Description|
|`endpoint`
`string`|
Endpoint of the collector this component will report traces to.
The connection is insecure, and does not currently support TLS.
Recommended is unset, and endpoint is the otlp grpc default, localhost:4317.
|
|`samplingRatePerMillion`
`int32`|
SamplingRatePerMillion is the number of samples to collect per million spans.
Recommended is unset. If unset, sampler respects its parent span's sampling
rate, but otherwise never samples.
|
## `AuthenticationConfiguration`
AuthenticationConfiguration provides versioned configuration for authentication.
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`AuthenticationConfiguration`|
|`jwt`**[Required]**
[`[]JWTAuthenticator`](#apiserver-k8s-io-v1beta1-JWTAuthenticator)|
jwt is a list of authenticator to authenticate Kubernetes users using
JWT compliant tokens. The authenticator will attempt to parse a raw ID token,
verify it's been signed by the configured issuer. The public key to verify the
signature is discovered from the issuer's public endpoint using OIDC discovery.
For an incoming token, each JWT authenticator will be attempted in
the order in which it is specified in this list. Note however that
other authenticators may run before or after the JWT authenticators.
The specific position of JWT authenticators in relation to other
authenticators is neither defined nor stable across releases. Since
each JWT authenticator must have a unique issuer URL, at most one
JWT authenticator will attempt to cryptographically validate the token.
The minimum valid JWT payload must contain the following claims:
{
"iss": "https://issuer.example.com",
"aud": ["audience"],
"exp": 1234567890,
"": "username"
}
|
|`anonymous`**[Required]**
[`AnonymousAuthConfig`](#apiserver-k8s-io-v1beta1-AnonymousAuthConfig)|
If present --anonymous-auth must not be set
|
## `AuthorizationConfiguration`
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`AuthorizationConfiguration`|
|`authorizers`**[Required]**
[`[]AuthorizerConfiguration`](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|
Authorizers is an ordered list of authorizers to
authorize requests against.
This is similar to the --authorization-modes kube-apiserver flag
Must be at least one.
|
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