---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 114
summary: It is important to note that Kubernetes does not have an in-built user database within the cluster. Instead, it takes user information from the configured authentication system and uses that to make...
---

It is important to note that Kubernetes does not have an in-built user database within the cluster.
Instead, it takes user information from the configured authentication system and uses that to make
authorization decisions. Therefore, to audit user access, you need to review credentials from every
configured authentication source.
For production clusters with multiple users directly accessing the Kubernetes API, it is
recommended to use external authentication sources such as OIDC. The internal authentication
mechanisms, such as client certificates and service account tokens, described below, are not
suitable for this use case.