---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#6-detailed
chunk_level: detailed
chunk_type: table
heading: `Issuer`
token_count: 982
summary: ## `Issuer` **Appears in:** * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) Issuer provides the configuration for an external provider's specific settings. |Field|Description|...
---

## `Issuer`
**Appears in:**
* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
Issuer provides the configuration for an external provider's specific settings.
|Field|Description|
|`url`**[Required]**
`string`|
url points to the issuer URL in a format https://url or https://url/path.
This must match the "iss" claim in the presented JWT, and the issuer returned from discovery.
Same value as the --oidc-issuer-url flag.
Discovery information is fetched from "{url}/.well-known/openid-configuration" unless overridden by discoveryURL.
Required to be unique across all JWT authenticators.
Note that egress selection configuration is not used for this network connection.
|
|`discoveryURL`
`string`|
discoveryURL, if specified, overrides the URL used to fetch discovery
information instead of using "{url}/.well-known/openid-configuration".
The exact value specified is used, so "/.well-known/openid-configuration"
must be included in discoveryURL if needed.
The "issuer" field in the fetched discovery information must match the "issuer.url" field
in the AuthenticationConfiguration and will be used to validate the "iss" claim in the presented JWT.
This is for scenarios where the well-known and jwks endpoints are hosted at a different
location than the issuer (such as locally in the cluster).
Example:
A discovery url that is exposed using kubernetes service 'oidc' in namespace 'oidc-namespace'
and discovery information is available at '/.well-known/openid-configuration'.
discoveryURL: "https://oidc.oidc-namespace/.well-known/openid-configuration"
certificateAuthority is used to verify the TLS connection and the hostname on the leaf certificate
must be set to 'oidc.oidc-namespace'.
curl https://oidc.oidc-namespace/.well-known/openid-configuration (.discoveryURL field)
{
issuer: "https://oidc.example.com" (.url field)
}
discoveryURL must be different from url.
Required to be unique across all JWT authenticators.
Note that egress selection configuration is not used for this network connection.
|
|`certificateAuthority`
`string`|
certificateAuthority contains PEM-encoded certificate authority certificates
used to validate the connection when fetching discovery information.
If unset, the system verifier is used.
Same value as the content of the file referenced by the --oidc-ca-file flag.
|
|`audiences`**[Required]**
`[]string`|
audiences is the set of acceptable audiences the JWT must be issued to.
At least one of the entries must match the "aud" claim in presented JWTs.
Same value as the --oidc-client-id flag (though this field supports an array).
Required to be non-empty.
|
|`audienceMatchPolicy`
[`AudienceMatchPolicyType`](#apiserver-k8s-io-v1beta1-AudienceMatchPolicyType)|
audienceMatchPolicy defines how the "audiences" field is used to match the "aud" claim in the presented JWT.
Allowed values are:
1. "MatchAny" when multiple audiences are specified and
2. empty (or unset) or "MatchAny" when a single audience is specified.
* MatchAny: the "aud" claim in the presented JWT must match at least one of the entries in the "audiences" field.
For example, if "audiences" is ["foo", "bar"], the "aud" claim in the presented JWT must contain either "foo" or "bar" (and may contain both).
* "": The match policy can be empty (or unset) when a single audience is specified in the "audiences" field. The "aud" claim in the presented JWT must contain the single audience (and may contain others).
For more nuanced audience validation, use claimValidationRules.
example: claimValidationRule[].expression: 'sets.equivalent(claims.aud, ["bar", "foo", "baz"])' to require an exact match.
|
|`egressSelectorType`
[`EgressSelectorType`](#apiserver-k8s-io-v1beta1-EgressSelectorType)|
egressSelectorType is an indicator of which egress selection should be used for sending all traffic related
to this issuer (discovery, JWKS, distributed claims, etc). If unspecified, no custom dialer is used.
When specified, the valid choices are "controlplane" and "cluster". These correspond to the associated
values in the --egress-selector-config-file.
* controlplane: for traffic intended to go to the control plane.
* cluster: for traffic intended to go to the system being managed by Kubernetes.
|