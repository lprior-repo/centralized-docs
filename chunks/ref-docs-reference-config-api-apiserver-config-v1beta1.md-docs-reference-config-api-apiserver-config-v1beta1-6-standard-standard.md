---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#6-standard
chunk_level: standard
chunk_type: table
heading: `ClaimMappings`
token_count: 499
summary: * [JWTAuthenticator](#apiserver-k8s-io-v1beta1-JWTAuthenticator) ClaimMappings provides the configuration for claim mapping |Field|Description| |`username`**[Required]**...
---

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