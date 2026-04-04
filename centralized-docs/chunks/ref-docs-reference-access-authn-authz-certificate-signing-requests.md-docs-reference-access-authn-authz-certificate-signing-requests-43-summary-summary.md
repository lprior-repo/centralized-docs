---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#43-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 109
summary: 6. CA bit allowed/disallowed - not allowed. 7. `kubernetes.io/legacy-unknown`: has no guarantees for trust at all. Some third-party distributions of Kubernetes may honor client certificates signed by...
---

6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/legacy-unknown`: has no guarantees for trust at all. Some third-party distributions of Kubernetes
may honor client certificates signed by it. The stable CertificateSigningRequest API (version `certificates.k8s.io/v1` and later)
does not allow to set the `signerName` as `kubernetes.io/legacy-unknown`.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).