---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#9-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 104
summary: * There is no lockout mechanism available to prevent brute-force attacks.## Bootstrap tokens [Bootstrap tokens](/docs/reference/access-authn-authz/bootstrap-tokens/) are used for joining nodes to...
---

* There is no lockout mechanism available to prevent brute-force attacks.## Bootstrap tokens
[Bootstrap tokens](/docs/reference/access-authn-authz/bootstrap-tokens/) are used for joining
nodes to clusters and are not recommended for user authentication due to several reasons:
* They have hard-coded group memberships that are not suitable for general use, making them
unsuitable for authentication purposes.
* Manually generating bootstrap tokens can lead to weak tokens that can be guessed by an attacker,
which can be a security risk.