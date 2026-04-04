---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#81-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 122
summary: * Verbs: **sign**, group: `certificates.k8s.io`, resource: `signers`, resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*` The signing controller is free...
---

* Verbs: **sign**, group: `certificates.k8s.io`, resource: `signers`,
resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
The signing controller is free to consider other information beyond what's
contained in the request, but it can rely on the information in the request to
be accurate. For example, the signing controller might load the Pod and read
annotations set on it, or perform a SubjectAccessReview on the ServiceAccount.