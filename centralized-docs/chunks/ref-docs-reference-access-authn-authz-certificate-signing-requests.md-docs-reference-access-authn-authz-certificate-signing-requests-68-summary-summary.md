---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#68-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 111
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