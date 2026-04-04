---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#12-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 113
summary: * Service accounts cannot be added to arbitrary groups complicating RBAC management where they are used.## TokenRequest API tokens The TokenRequest API is a useful tool for generating short-lived...
---

* Service accounts cannot be added to arbitrary groups complicating RBAC management where they are used.## TokenRequest API tokens
The TokenRequest API is a useful tool for generating short-lived credentials for service
authentication to the API server or third-party systems. However, it is not generally recommended
for user authentication as there is no revocation method available, and distributing credentials
to users in a secure manner can be challenging.
When using TokenRequest tokens for service authentication, it is recommended to implement a short
lifespan to reduce the impact of compromised tokens.