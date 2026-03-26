---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#30-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 114
summary: &gt;#\". For the same behavior using authentication config, set username.prefix=\"#\" (3) --oidc-username-prefix=\"\". For the same behavior using authentication config, set username.prefix=\"\" | |`groups`...
---

&gt;#". For the same
behavior using authentication config, set username.prefix="#"
(3) --oidc-username-prefix="". For the same behavior using authentication config, set username.prefix=""
|
|`groups`
[`PrefixedClaimOrExpression`](#apiserver-k8s-io-v1beta1-PrefixedClaimOrExpression)|
groups represents an option for the groups attribute.
The claim's value must be a string or string array claim.
If groups.claim is set, the prefix must be specified (and can be the empty string).