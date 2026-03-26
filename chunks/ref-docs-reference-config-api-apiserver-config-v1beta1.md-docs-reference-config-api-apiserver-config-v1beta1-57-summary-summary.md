---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#57-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 125
summary: --oidc-ca-file flag. | |`audiences`**[Required]** `[]string`| audiences is the set of acceptable audiences the JWT must be issued to. At least one of the entries must match the \"aud\" claim in...
---

--oidc-ca-file flag.
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
audienceMatchPolicy defines how the "audiences"