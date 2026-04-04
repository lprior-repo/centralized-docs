---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#83-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 126
summary: * Suggests a time to begin attempting to refresh the certificate using `status.beginRefreshAt`. To deny a request, the signing controller adds a \"Denied\" condition to `status.conditions[]`. To mark a...
---

* Suggests a time to begin attempting to refresh the certificate using
`status.beginRefreshAt`.
To deny a request, the signing controller adds a "Denied" condition to
`status.conditions[]`.
To mark a request failed, the signing controller adds a "Failed" condition to
`status.conditions[]`.
All of these conditions are mutually-exclusive, and must have status "True". No
other condition types are permitted on PodCertificateRequests. In addition,
once any of these conditions are set, the `status` field becomes immutable.
Like all conditions, the `status.conditions[].reason` field is meant to contain