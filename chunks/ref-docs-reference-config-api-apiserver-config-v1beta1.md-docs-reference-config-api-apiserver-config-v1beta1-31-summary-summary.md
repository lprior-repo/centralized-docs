---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#31-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 115
summary: If groups.claim is set, the prefix must be specified (and can be the empty string). If groups.expression is set, the expression must produce a string or string array value. \"\", [], and null values...
---

If groups.claim is set, the prefix must be specified (and can be the empty string).
If groups.expression is set, the expression must produce a string or string array value.
"", [], and null values are treated as the group mapping not being present.
|
|`uid`
[`ClaimOrExpression`](#apiserver-k8s-io-v1beta1-ClaimOrExpression)|
uid represents an option for the uid attribute.
Claim must be a singular string claim.
If uid.expression is set, the expression must produce a string value.
|
|`extra`