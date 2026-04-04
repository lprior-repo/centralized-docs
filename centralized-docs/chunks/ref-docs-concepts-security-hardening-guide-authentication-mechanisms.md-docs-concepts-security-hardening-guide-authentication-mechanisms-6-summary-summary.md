---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#6-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 64
summary: * Private keys used for client certificate authentication cannot be password-protected. Anyone who can read the file containing the key will be able to make use of it. * Using client certificate...
---

* Private keys used for client certificate authentication cannot be password-protected. Anyone
who can read the file containing the key will be able to make use of it.
* Using client certificate authentication requires a direct connection from the client to the
API server without any intervening TLS termination points, which can complicate network architectures.