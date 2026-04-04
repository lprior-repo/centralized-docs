---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#8-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 78
summary: * Credentials are stored in clear text on control plane node disks, which can be a security risk. * Changing any credential requires a restart of the API server process to take effect, which can...
---

* Credentials are stored in clear text on control plane node disks, which can be a security risk.
* Changing any credential requires a restart of the API server process to take effect, which can
impact availability.
* There is no mechanism available to allow users to rotate their credentials. To rotate a
credential, a cluster administrator must modify the token on disk and distribute it to the users.