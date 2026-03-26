---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#3-detailed
chunk_level: detailed
chunk_type: table
heading: `ClaimMappings`
token_count: 843
summary: ## `ClaimMappings` **Appears in:** * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) ClaimMappings provides the configuration for claim mapping |Field|Description|...
---

## `ClaimMappings`
**Appears in:**
* [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator)
ClaimMappings provides the configuration for claim mapping
|Field|Description|
|`username`**[Required]**
[`PrefixedClaimOrExpression`](#apiserver-k8s-io-v1beta1-PrefixedClaimOrExpression)|
username represents an option for the username attribute.
The claim's value must be a singular string.
Same as the --oidc-username-claim and --oidc-username-prefix flags.
If username.expression is set, the expression must produce a string value.
If username.expression uses 'claims.email', then 'claims.email\_verified' must be used in
username.expression or extra[*].valueExpression or claimValidationRules[*].expression.
An example claim validation rule expression that matches the validation automatically
applied when username.claim is set to 'email' is 'claims.?email\_verified.orValue(true) == true'. By explicitly comparing
the value to true, we let type-checking see the result will be a boolean, and to make sure a non-boolean email\_verified
claim will be caught at runtime.
In the flag based approach, the --oidc-username-claim and --oidc-username-prefix are optional. If --oidc-username-claim is not set,
the default value is "sub". For the authentication config, there is no defaulting for claim or prefix. The claim and prefix must be set explicitly.
For claim, if --oidc-username-claim was not set with legacy flag approach, configure username.claim="sub" in the authentication config.
For prefix:
(1) --oidc-username-prefix="-", no prefix was added to the username. For the same behavior using authentication config,
set username.prefix=""
(2) --oidc-username-prefix="" and --oidc-username-claim != "email", prefix was "&lt;value of --oidc-issuer-url&gt;#". For the same
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