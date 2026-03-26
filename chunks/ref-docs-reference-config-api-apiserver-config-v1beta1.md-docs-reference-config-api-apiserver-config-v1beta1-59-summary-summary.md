---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#59-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 69
summary: * MatchAny: the \"aud\" claim in the presented JWT must match at least one of the entries in the \"audiences\" field. For example, if \"audiences\" is [\"foo\", \"bar\"], the \"aud\" claim in the presented JWT...
---

* MatchAny: the "aud" claim in the presented JWT must match at least one of the entries in the "audiences" field.
For example, if "audiences" is ["foo", "bar"], the "aud" claim in the presented JWT must contain either "foo" or "bar" (and may contain both).