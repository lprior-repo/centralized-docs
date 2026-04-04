---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#11-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 73
summary: with TokenRequest API tokens. While these secrets could be used for user authentication, they are generally unsuitable for a number of reasons: * They cannot be set with an expiry and will remain...
---

with TokenRequest API tokens. While these secrets could be used for user authentication, they are
generally unsuitable for a number of reasons:
* They cannot be set with an expiry and will remain valid until the associated service account is deleted.
* The authentication tokens are visible to any cluster user who can read secrets in the namespace
that they are defined in.