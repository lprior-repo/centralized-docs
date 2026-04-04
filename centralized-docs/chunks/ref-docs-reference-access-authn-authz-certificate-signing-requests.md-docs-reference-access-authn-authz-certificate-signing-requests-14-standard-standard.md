---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#14-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 131
summary: ### API-based signers Users of the REST API can sign CSRs by submitting an UPDATE request to the `status` subresource of the CSR to be signed. As part of this request, the `status.certificate` field...
---

### API-based signers
Users of the REST API can sign CSRs by submitting an UPDATE request to the `status`
subresource of the CSR to be signed.
As part of this request, the `status.certificate` field should be set to contain the
signed certificate. This field contains one or more PEM-encoded certificates.
All PEM blocks must have the "CERTIFICATE" label, contain no headers,
and the encoded data must be a BER-encoded ASN.1 Certificate structure
as described in [section 4 of RFC5280](https://tools.ietf.org/html/rfc5280#section-4.1).
Example certificate content: