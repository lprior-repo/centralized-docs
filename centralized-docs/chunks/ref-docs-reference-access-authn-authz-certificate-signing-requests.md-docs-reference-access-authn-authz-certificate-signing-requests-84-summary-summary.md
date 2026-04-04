---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#84-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 107
summary: `status` field becomes immutable. Like all conditions, the `status.conditions[].reason` field is meant to contain a machine-readable code describing the condition in TitleCase. The...
---

`status` field becomes immutable.
Like all conditions, the `status.conditions[].reason` field is meant to contain
a machine-readable code describing the condition in TitleCase. The
`status.conditions[].message` field is meant for a free-form explanation for
human consumption.
To ensure that terminal PodCertificateRequests do not build up in the cluster, a
`kube-controller-manager` controller deletes all PodCertificateRequests older
than 15 minutes. All certificate issuance flows are expected to complete within
this 15-minute limit.