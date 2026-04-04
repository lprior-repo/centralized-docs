---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#5-standard
chunk_level: standard
chunk_type: prose
heading: OpenID Connect token authentication
token_count: 329
summary: ## OpenID Connect token authentication Kubernetes supports integrating external authentication services with the Kubernetes API using [OpenID Connect...
---

## OpenID Connect token authentication
Kubernetes supports integrating external authentication services with the Kubernetes API using
[OpenID Connect (OIDC)](/docs/reference/access-authn-authz/authentication/#openid-connect-tokens).
There is a wide variety of software that can be used to integrate Kubernetes with an identity
provider. However, when using OIDC authentication in Kubernetes, it is important to consider the
following hardening measures:
* The software installed in the cluster to support OIDC authentication should be isolated from
general workloads as it will run with high privileges.
* Some Kubernetes managed services are limited in the OIDC providers that can be used.
* As with TokenRequest tokens, OIDC tokens should have a short lifespan to reduce the impact of
compromised tokens.## Webhook token authentication
[Webhook token authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication)
is another option for integrating external authentication providers into Kubernetes. This mechanism
allows for an authentication service, either running inside the cluster or externally, to be
contacted for an authentication decision over a webhook. It is important to note that the suitability
of this mechanism will likely depend on the software used for the authentication service, and there
are some Kubernetes-specific considerations to take into account.
To configure Webhook authentication, access to control plane server filesystems is required. This
means that it will not be possible with Managed Kubernetes unless the provider specifically makes it
available. Additionally, any software installed in the cluster to support this access should be
isolated from general workloads, as it will run with high privileges.