---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#28-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 117
summary: 1. **Trust distribution**: how trust anchors (CA certificates or certificate bundles) are distributed. 2. **Permitted subjects**: any restrictions on and behavior when a disallowed subject is...
---

1. **Trust distribution**: how trust anchors (CA certificates or certificate bundles) are distributed.
2. **Permitted subjects**: any restrictions on and behavior when a disallowed subject is requested.
3. **Permitted x509 extensions**: including IP subjectAltNames, DNS subjectAltNames,
Email subjectAltNames, URI subjectAltNames etc, and behavior when a disallowed extension is requested.
4. **Permitted key usages / extended key usages**: any restrictions on and behavior
when usages different than the signer-determined usages are specified in the CSR.