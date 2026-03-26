---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#58-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 63
summary: | audienceMatchPolicy defines how the \"audiences\" field is used to match the \"aud\" claim in the presented JWT. Allowed values are: 1. \"MatchAny\" when multiple audiences are specified and 2. empty (or...
---

|
audienceMatchPolicy defines how the "audiences" field is used to match the "aud" claim in the presented JWT.
Allowed values are:
1. "MatchAny" when multiple audiences are specified and
2. empty (or unset) or "MatchAny" when a single audience is specified.