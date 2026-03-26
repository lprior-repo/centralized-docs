---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#40-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimValidationRule`
token_count: 120
summary: * 'claims' is a map of claim names to claim values. For example, a variable named 'sub' can be accessed as 'claims.sub'. Nested claims can be accessed using dot notation, e.g. 'claims.foo.bar'. Must...
---

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