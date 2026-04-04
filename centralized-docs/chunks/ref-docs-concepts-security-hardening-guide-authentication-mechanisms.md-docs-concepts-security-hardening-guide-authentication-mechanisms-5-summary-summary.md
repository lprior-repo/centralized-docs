---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#5-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 108
summary: * Client certificates cannot be individually revoked. Once compromised, a certificate can be used by an attacker until it expires. To mitigate this risk, it is recommended to configure short...
---

* Client certificates cannot be individually revoked. Once compromised, a certificate can be used
by an attacker until it expires. To mitigate this risk, it is recommended to configure short
lifetimes for user authentication credentials created using client certificates.
* If a certificate needs to be invalidated, the certificate authority must be re-keyed, which
can introduce availability risks to the cluster.
* There is no permanent record of client certificates created in the cluster. Therefore, all
issued certificates must be recorded if you need to keep track of them.