---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#70-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 101
summary: For `Denied` CSRs: ``` `apiVersion: certificates.k8s.io/v1 kind: CertificateSigningRequest ... status: conditions: - lastUpdateTime: \"2020-02-08T11:37:35Z\" lastTransitionTime: \"2020-02-08T11:37:35Z\"...
---

For `Denied` CSRs:
```
`apiVersion: certificates.k8s.io/v1
kind: CertificateSigningRequest
...
status:
conditions:
- lastUpdateTime: "2020-02-08T11:37:35Z"
lastTransitionTime: "2020-02-08T11:37:35Z"
message: Denied by my custom approver controller
reason: DeniedByMyPolicy # You can set this to any string
type: Denied
`
```