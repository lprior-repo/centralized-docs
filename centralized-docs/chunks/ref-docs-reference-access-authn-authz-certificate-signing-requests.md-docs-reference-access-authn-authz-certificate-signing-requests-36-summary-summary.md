---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#36-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 111
summary: 1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle is not distributed by any other means. 2. Permitted subjects - no subject...
---

1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle is not distributed by any other means.
2. Permitted subjects - no subject restrictions, but approvers and signers may choose not to approve or sign.
Certain subjects like cluster-admin level users or groups vary between distributions and installations,
but deserve additional scrutiny before approval and signing.
The `CertificateSubjectRestriction` admission plugin is enabled by default to restrict `system:masters`,
but it is often not the only cluster-admin subject in a cluster.