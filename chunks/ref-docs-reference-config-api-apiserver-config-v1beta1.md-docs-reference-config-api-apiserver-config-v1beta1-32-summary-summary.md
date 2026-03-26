---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#32-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 120
summary: Claim must be a singular string claim. If uid.expression is set, the expression must produce a string value. | |`extra` [`[]ExtraMapping`](#apiserver-k8s-io-v1beta1-ExtraMapping)| extra represents an...
---

Claim must be a singular string claim.
If uid.expression is set, the expression must produce a string value.
|
|`extra`
[`[]ExtraMapping`](#apiserver-k8s-io-v1beta1-ExtraMapping)|
extra represents an option for the extra attribute.
expression must produce a string or string array value.
If the value is empty, the extra mapping will not be present.
hard-coded extra key/value
* key: "foo"
valueExpression: "'bar'"
This will result in an extra attribute - foo: ["bar"]
hard-coded key, value copying claim value