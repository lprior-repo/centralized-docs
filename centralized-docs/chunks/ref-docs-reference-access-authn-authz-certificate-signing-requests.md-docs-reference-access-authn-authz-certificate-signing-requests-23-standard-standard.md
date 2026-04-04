---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#23-standard
chunk_level: standard
chunk_type: prose
heading: Approval or rejection
token_count: 450
summary: * Verbs: **update**, group: `certificates.k8s.io`, resource: `podcertificaterequests/status` * Verbs: **sign**, group: `certificates.k8s.io`, resource: `signers`, resourceName:...
---

* Verbs: **update**, group: `certificates.k8s.io`, resource:
`podcertificaterequests/status`
* Verbs: **sign**, group: `certificates.k8s.io`, resource: `signers`,
resourceName: `&lt;signerNameDomain&gt;/&lt;signerNamePath&gt;` or `&lt;signerNameDomain&gt;/\*`
The signing controller is free to consider other information beyond what's
contained in the request, but it can rely on the information in the request to
be accurate. For example, the signing controller might load the Pod and read
annotations set on it, or perform a SubjectAccessReview on the ServiceAccount.
To issue a certificate in response to a request, the signing controller:
* Adds an `Issued` condition to `status.conditions`.
* Puts the issued certificate in `status.certificateChain`
* Puts the `NotBefore` and `NotAfter` fields of the certificate in the
`status.notBefore` and `status.notAfter` fields — these fields are
denormalized into the Kubernetes API in order to aid debugging
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
a machine-readable code describing the condition in TitleCase. The
`status.conditions[].message` field is meant for a free-form explanation for
human consumption.
To ensure that terminal PodCertificateRequests do not build up in the cluster, a
`kube-controller-manager` controller deletes all PodCertificateRequests older
than 15 minutes. All certificate issuance flows are expected to complete within
this 15-minute limit.