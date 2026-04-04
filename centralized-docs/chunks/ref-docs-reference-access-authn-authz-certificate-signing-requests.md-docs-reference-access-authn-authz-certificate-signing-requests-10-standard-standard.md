---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#10-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 507
summary: 1. `kubernetes.io/kube-apiserver-client`: signs certificates that will be honored as client certificates by the API server. Never auto-approved by...
---

1. `kubernetes.io/kube-apiserver-client`: signs certificates that will be honored as client certificates by the API server.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle is not distributed by any other means.
2. Permitted subjects - no subject restrictions, but approvers and signers may choose not to approve or sign.
Certain subjects like cluster-admin level users or groups vary between distributions and installations,
but deserve additional scrutiny before approval and signing.
The `CertificateSubjectRestriction` admission plugin is enabled by default to restrict `system:masters`,
but it is often not the only cluster-admin subject in a cluster.
3. Permitted x509 extensions - honors subjectAltName and key usage extensions and discards other extensions.
4. Permitted key usages - must include `["client auth"]`. Must not include key usages beyond `["digital signature", "key encipherment", "client auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/kube-apiserver-client-kubelet`: signs client certificates that will be honored as client certificates by the
API server.
May be auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle
is not distributed by any other means.
2. Permitted subjects - organizations are exactly `["system:nodes"]`, common name is "`system:node:${NODE\_NAME}`".
3. Permitted x509 extensions - honors key usage extensions, forbids subjectAltName extensions and drops other extensions.
4. Permitted key usages - `["key encipherment", "digital signature", "client auth"]` or `["digital signature", "client auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.