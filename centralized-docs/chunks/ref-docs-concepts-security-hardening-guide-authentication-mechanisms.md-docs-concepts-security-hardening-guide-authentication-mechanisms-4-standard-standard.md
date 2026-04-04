---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#4-standard
chunk_level: standard
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 339
summary: * They have hard-coded group memberships that are not suitable for general use, making them unsuitable for authentication purposes. * Manually generating bootstrap tokens can lead to weak tokens that...
---

* They have hard-coded group memberships that are not suitable for general use, making them
unsuitable for authentication purposes.
* Manually generating bootstrap tokens can lead to weak tokens that can be guessed by an attacker,
which can be a security risk.
* There is no lockout mechanism available to prevent brute-force attacks, making it easier for
attackers to guess or crack the token.## ServiceAccount secret tokens
[Service account secrets](/docs/reference/access-authn-authz/service-accounts-admin/#manual-secret-management-for-serviceaccounts)
are available as an option to allow workloads running in the cluster to authenticate to the
API server. In Kubernetes &lt; 1.23, these were the default option, however, they are being replaced
with TokenRequest API tokens. While these secrets could be used for user authentication, they are
generally unsuitable for a number of reasons:
* They cannot be set with an expiry and will remain valid until the associated service account is deleted.
* The authentication tokens are visible to any cluster user who can read secrets in the namespace
that they are defined in.
* Service accounts cannot be added to arbitrary groups complicating RBAC management where they are used.## TokenRequest API tokens
The TokenRequest API is a useful tool for generating short-lived credentials for service
authentication to the API server or third-party systems. However, it is not generally recommended
for user authentication as there is no revocation method available, and distributing credentials
to users in a secure manner can be challenging.
When using TokenRequest tokens for service authentication, it is recommended to implement a short
lifespan to reduce the impact of compromised tokens.