---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#13-standard
chunk_level: standard
chunk_type: prose
heading: `Issuer`
token_count: 263
summary: (.url field) } discoveryURL must be different from url. Required to be unique across all JWT authenticators. Note that egress selection configuration is not used for this network connection. |...
---

 (.url field)
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