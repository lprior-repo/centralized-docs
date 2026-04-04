---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#64-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 86
summary: Non-PEM content may appear before or after the CERTIFICATE PEM blocks and is unvalidated, to allow for explanatory text as described in [section 5.2 of...
---

Non-PEM content may appear before or after the CERTIFICATE PEM blocks and is unvalidated,
to allow for explanatory text as described in [section 5.2 of RFC7468](https://www.rfc-editor.org/rfc/rfc7468#section-5.2).
When encoded in JSON or YAML, this field is base-64 encoded.
A CertificateSigningRequest containing the example certificate above would look like this: