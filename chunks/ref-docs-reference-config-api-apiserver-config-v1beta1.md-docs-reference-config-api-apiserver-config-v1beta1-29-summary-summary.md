---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#29-summary
chunk_level: summary
chunk_type: prose
heading: `ClaimMappings`
token_count: 128
summary: For claim, if --oidc-username-claim was not set with legacy flag approach, configure username.claim=\"sub\" in the authentication config. For prefix: (1) --oidc-username-prefix=\"-\", no prefix was added...
---

For claim, if --oidc-username-claim was not set with legacy flag approach, configure username.claim="sub" in the authentication config.
For prefix:
(1) --oidc-username-prefix="-", no prefix was added to the username. For the same behavior using authentication config,
set username.prefix=""
(2) --oidc-username-prefix="" and --oidc-username-claim != "email", prefix was "&lt;value of --oidc-issuer-url&gt;#". For the same
behavior using authentication config, set username.prefix="#"
(3) --oidc-username-prefix=""