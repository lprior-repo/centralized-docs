---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#33-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 124
summary: * key: \"foo\" valueExpression: \"claims.some\_claim\" This will result in an extra attribute - foo: [value of some\_claim] hard-coded key, value derived from claim value * key: \"admin\" valueExpression:...
---

* key: "foo"
valueExpression: "claims.some\_claim"
This will result in an extra attribute - foo: [value of some\_claim]
hard-coded key, value derived from claim value
* key: "admin"
valueExpression: '(has(claims.is\_admin) &amp;&amp; claims.is\_admin) ? "true":""'
This will result in:
* if is\_admin claim is present and true, extra attribute - admin: ["true"]
* if is\_admin claim is present and false or is\_admin claim is not present, no extra attribute will be added|