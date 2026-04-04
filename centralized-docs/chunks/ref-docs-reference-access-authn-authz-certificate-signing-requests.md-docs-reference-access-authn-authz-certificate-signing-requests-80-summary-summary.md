---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#80-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 100
summary: controller directly decides to issue or deny the request. It also has the option to mark the request as failed, if it encountered a permanent error when attempting to issue the request. To take any...
---

controller directly decides to issue or deny the request. It also has the
option to mark the request as failed, if it encountered a permanent error when
attempting to issue the request.
To take any of these actions, the signing controller needs to have the
appropriate permissions on both the PodCertificateRequest type, as well as on
the signer name:
* Verbs: **update**, group: `certificates.k8s.io`, resource:
`podcertificaterequests/status`