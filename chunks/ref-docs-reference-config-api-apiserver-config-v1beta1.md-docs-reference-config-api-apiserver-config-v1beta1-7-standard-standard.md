---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#7-standard
chunk_level: standard
chunk_type: prose
heading: `ClaimMappings`
token_count: 433
summary: --oidc-issuer-url&gt;#\". For the same behavior using authentication config, set username.prefix=\"#\" (3) --oidc-username-prefix=\"\". For the same behavior using authentication config, set...
---

--oidc-issuer-url&gt;#". For the same
behavior using authentication config, set username.prefix="#"
(3) --oidc-username-prefix="". For the same behavior using authentication config, set username.prefix=""
|
|`groups`
[`PrefixedClaimOrExpression`](#apiserver-k8s-io-v1beta1-PrefixedClaimOrExpression)|
groups represents an option for the groups attribute.
The claim's value must be a string or string array claim.
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
[`[]ExtraMapping`](#apiserver-k8s-io-v1beta1-ExtraMapping)|
extra represents an option for the extra attribute.
expression must produce a string or string array value.
If the value is empty, the extra mapping will not be present.
hard-coded extra key/value
* key: "foo"
valueExpression: "'bar'"
This will result in an extra attribute - foo: ["bar"]
hard-coded key, value copying claim value
* key: "foo"
valueExpression: "claims.some\_claim"
This will result in an extra attribute - foo: [value of some\_claim]
hard-coded key, value derived from claim value
* key: "admin"
valueExpression: '(has(claims.is\_admin) &amp;&amp; claims.is\_admin) ? "true":""'
This will result in:
* if is\_admin claim is present and true, extra attribute - admin: ["true"]
* if is\_admin claim is present and false or is\_admin claim is not present, no extra attribute will be added|