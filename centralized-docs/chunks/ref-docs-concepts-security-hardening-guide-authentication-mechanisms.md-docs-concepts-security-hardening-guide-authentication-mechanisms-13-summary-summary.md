---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#13-summary
chunk_level: summary
chunk_type: prose
heading: OpenID Connect token authentication
token_count: 85
summary: ## OpenID Connect token authentication Kubernetes supports integrating external authentication services with the Kubernetes API using [OpenID Connect...
---

## OpenID Connect token authentication
Kubernetes supports integrating external authentication services with the Kubernetes API using
[OpenID Connect (OIDC)](/docs/reference/access-authn-authz/authentication/#openid-connect-tokens).
There is a wide variety of software that can be used to integrate Kubernetes with an identity
provider. However, when using OIDC authentication in Kubernetes, it is important to consider the
following hardening measures: