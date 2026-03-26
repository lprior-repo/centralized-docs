---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#60-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 121
summary: * \"\": The match policy can be empty (or unset) when a single audience is specified in the \"audiences\" field. The \"aud\" claim in the presented JWT must contain the single audience (and may contain...
---

* "": The match policy can be empty (or unset) when a single audience is specified in the "audiences" field. The "aud" claim in the presented JWT must contain the single audience (and may contain others).
For more nuanced audience validation, use claimValidationRules.
example: claimValidationRule[].expression: 'sets.equivalent(claims.aud, ["bar", "foo", "baz"])' to require an exact match.
|
|`egressSelectorType`
[`EgressSelectorType`](#apiserver-k8s-io-v1beta1-EgressSelectorType)|