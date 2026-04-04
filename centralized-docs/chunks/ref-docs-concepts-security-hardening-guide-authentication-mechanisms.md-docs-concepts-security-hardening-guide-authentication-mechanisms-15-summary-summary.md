---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#15-summary
chunk_level: summary
chunk_type: prose
heading: OpenID Connect token authentication
token_count: 121
summary: * As with TokenRequest tokens, OIDC tokens should have a short lifespan to reduce the impact of compromised tokens.## Webhook token authentication [Webhook token...
---

* As with TokenRequest tokens, OIDC tokens should have a short lifespan to reduce the impact of
compromised tokens.## Webhook token authentication
[Webhook token authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication)
is another option for integrating external authentication providers into Kubernetes. This mechanism
allows for an authentication service, either running inside the cluster or externally, to be
contacted for an authentication decision over a webhook. It is important to note that the suitability
of this mechanism will likely depend on the software used for the authentication service, and there