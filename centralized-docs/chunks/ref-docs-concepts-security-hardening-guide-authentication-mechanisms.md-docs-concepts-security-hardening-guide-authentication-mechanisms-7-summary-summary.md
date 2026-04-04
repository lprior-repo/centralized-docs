---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#7-summary
chunk_level: summary
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 113
summary: * Group data is embedded in the `O` value of the client certificate, which means the user's group memberships cannot be changed for the lifetime of the certificate.## Static token file Although...
---

* Group data is embedded in the `O` value of the client certificate, which means the user's group
memberships cannot be changed for the lifetime of the certificate.## Static token file
Although Kubernetes allows you to load credentials from a
[static token file](/docs/reference/access-authn-authz/authentication/#static-token-file) located
on the control plane node disks, this approach is not recommended for production servers due to
several reasons:
* Credentials are stored in clear text on control plane node disks, which can be a security risk.