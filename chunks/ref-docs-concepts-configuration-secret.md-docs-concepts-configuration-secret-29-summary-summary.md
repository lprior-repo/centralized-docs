---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#29-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 52
summary: * For authentication, you can implement a custom signer for X.509 certificates, and use [CertificateSigningRequests](/docs/reference/access-authn-authz/certificate-signing-requests/) to let that...
---

* For authentication, you can implement a custom signer for X.509 certificates, and use
[CertificateSigningRequests](/docs/reference/access-authn-authz/certificate-signing-requests/)
to let that custom signer issue certificates to Pods that need them.