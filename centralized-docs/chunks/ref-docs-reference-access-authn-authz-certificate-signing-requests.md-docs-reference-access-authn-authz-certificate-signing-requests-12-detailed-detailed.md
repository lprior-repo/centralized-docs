---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#12-detailed
chunk_level: detailed
chunk_type: prose
heading: Approval or rejection
token_count: 365
summary: ### Approval or rejection using the Kubernetes API Users of the REST API can approve CSRs by submitting an UPDATE request to the `approval` subresource of the CSR to be approved. For example, you...
---

### Approval or rejection using the Kubernetes API
Users of the REST API can approve CSRs by submitting an UPDATE request to the `approval`
subresource of the CSR to be approved. For example, you could write an
[operator](/docs/concepts/extend-kubernetes/operator/) that watches for a particular
kind of CSR and then sends an UPDATE to approve them.
When you make an approval or rejection request, set either the `Approved` or `Denied`
status condition based on the state you determine:
For `Approved` CSRs:
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
It's usual to set `status.conditions.reason` to a machine-friendly reason
code using TitleCase; this is a convention but you can set it to anything
you like. If you want to add a note for human consumption, use the
`status.conditions.message` field.