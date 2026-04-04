---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#10-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 121
summary: * There is no lockout mechanism available to prevent brute-force attacks, making it easier for attackers to guess or crack the token.## ServiceAccount secret tokens [Service account...
---

* There is no lockout mechanism available to prevent brute-force attacks, making it easier for
attackers to guess or crack the token.## ServiceAccount secret tokens
[Service account secrets](/docs/reference/access-authn-authz/service-accounts-admin/#manual-secret-management-for-serviceaccounts)
are available as an option to allow workloads running in the cluster to authenticate to the
API server. In Kubernetes &lt; 1.23, these were the default option, however, they are being replaced
with TokenRequest API tokens. While these secrets could be used for user authentication, they are