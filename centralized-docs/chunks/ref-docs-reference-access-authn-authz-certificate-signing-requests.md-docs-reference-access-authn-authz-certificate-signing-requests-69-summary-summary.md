---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#69-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 102
summary: ``` `apiVersion: certificates.k8s.io/v1 kind: CertificateSigningRequest ... status: conditions: - lastUpdateTime: \"2020-02-08T11:37:35Z\" lastTransitionTime: \"2020-02-08T11:37:35Z\" message: Approved...
---

```
`apiVersion: certificates.k8s.io/v1
kind: CertificateSigningRequest
...
status:
conditions:
- lastUpdateTime: "2020-02-08T11:37:35Z"
lastTransitionTime: "2020-02-08T11:37:35Z"
message: Approved by my custom approver controller
reason: ApprovedByMyPolicy # You can set this to any string
type: Approved
`
```
For `Denied` CSRs: