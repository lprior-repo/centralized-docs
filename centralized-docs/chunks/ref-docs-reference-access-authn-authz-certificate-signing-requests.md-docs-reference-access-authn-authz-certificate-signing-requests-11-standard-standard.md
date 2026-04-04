---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#11-standard
chunk_level: standard
chunk_type: prose
heading: Signers
token_count: 493
summary: 4. Permitted key usages - `[\"key encipherment\", \"digital signature\", \"client auth\"]` or `[\"digital signature\", \"client auth\"]`. 5. Expiration/certificate lifetime - for the kube-controller-manager...
---

4. Permitted key usages - `["key encipherment", "digital signature", "client auth"]` or `["digital signature", "client auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/kubelet-serving`: signs serving certificates that are honored as a valid kubelet serving certificate
by the API server, but has no other guarantees.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: signed certificates must be honored by the API server as valid to terminate connections to a kubelet.
The CA bundle is not distributed by any other means.
2. Permitted subjects - organizations are exactly `["system:nodes"]`, common name is "`system:node:${NODE\_NAME}`".
3. Permitted x509 extensions - honors key usage and DNSName/IPAddress subjectAltName extensions, forbids EmailAddress and
URI subjectAltName extensions, drops other extensions. At least one DNS or IP subjectAltName must be present.
4. Permitted key usages - `["key encipherment", "digital signature", "server auth"]` or `["digital signature", "server auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/legacy-unknown`: has no guarantees for trust at all. Some third-party distributions of Kubernetes
may honor client certificates signed by it. The stable CertificateSigningRequest API (version `certificates.k8s.io/v1` and later)
does not allow to set the `signerName` as `kubernetes.io/legacy-unknown`.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: None. There is no standard trust or distribution for this signer in a Kubernetes cluster.
2. Permitted subjects - any