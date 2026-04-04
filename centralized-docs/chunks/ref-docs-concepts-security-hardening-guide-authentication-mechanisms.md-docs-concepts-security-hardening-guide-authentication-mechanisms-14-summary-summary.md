---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#14-summary
chunk_level: summary
chunk_type: prose
heading: OpenID Connect token authentication
token_count: 45
summary: * The software installed in the cluster to support OIDC authentication should be isolated from general workloads as it will run with high privileges. * Some Kubernetes managed services are limited in...
---

* The software installed in the cluster to support OIDC authentication should be isolated from
general workloads as it will run with high privileges.
* Some Kubernetes managed services are limited in the OIDC providers that can be used.