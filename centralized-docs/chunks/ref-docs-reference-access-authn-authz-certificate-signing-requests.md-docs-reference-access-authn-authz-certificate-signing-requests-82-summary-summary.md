---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#82-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 105
summary: annotations set on it, or perform a SubjectAccessReview on the ServiceAccount. To issue a certificate in response to a request, the signing controller: * Adds an `Issued` condition to...
---

annotations set on it, or perform a SubjectAccessReview on the ServiceAccount.
To issue a certificate in response to a request, the signing controller:
* Adds an `Issued` condition to `status.conditions`.
* Puts the issued certificate in `status.certificateChain`
* Puts the `NotBefore` and `NotAfter` fields of the certificate in the
`status.notBefore` and `status.notAfter` fields — these fields are
denormalized into the Kubernetes API in order to aid debugging