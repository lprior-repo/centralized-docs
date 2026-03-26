---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#28-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 128
summary: the value to true, we let type-checking see the result will be a boolean, and to make sure a non-boolean email\_verified claim will be caught at runtime. In the flag based approach, the...
---

the value to true, we let type-checking see the result will be a boolean, and to make sure a non-boolean email\_verified
claim will be caught at runtime.
In the flag based approach, the --oidc-username-claim and --oidc-username-prefix are optional. If --oidc-username-claim is not set,
the default value is "sub". For the authentication config, there is no defaulting for claim or prefix. The claim and prefix must be set explicitly.
For claim, if --oidc-username-claim was not set with legacy flag approach, configure username.claim="