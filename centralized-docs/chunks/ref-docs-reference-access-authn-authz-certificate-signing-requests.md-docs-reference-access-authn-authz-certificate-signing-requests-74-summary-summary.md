---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#74-summary
chunk_level: summary
chunk_type: prose
heading: Approval or rejection
token_count: 119
summary: workloads running as Pods within a cluster. The user typically does not interact with PodCertificateRequests directly, but uses [podCertificate projected volume...
---

workloads running as Pods within a cluster. The user typically does not
interact with PodCertificateRequests directly, but uses [podCertificate
projected volume sources](/docs/concepts/storage/projected-volumes/#podcertificate), which are a `kubelet`
feature that handles secure key provisioning and automatic certificate refresh.
The application inside the pod only needs to know how to read the certificates
from the filesystem.
PodCertificateRequests are similar to CertificateSigningRequests, but have a
simpler format enabled by their narrower use case.
A PodCertificateRequest has the following spec fields: