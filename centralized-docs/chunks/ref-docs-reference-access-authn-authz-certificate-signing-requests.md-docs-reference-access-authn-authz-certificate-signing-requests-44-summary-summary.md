---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#44-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 113
summary: 1. Trust distribution: None. There is no standard trust or distribution for this signer in a Kubernetes cluster. 2. Permitted subjects - any 3. Permitted x509 extensions - honors subjectAltName and...
---

1. Trust distribution: None. There is no standard trust or distribution for this signer in a Kubernetes cluster.
2. Permitted subjects - any
3. Permitted x509 extensions - honors subjectAltName and key usage extensions and discards other extensions.
4. Permitted key usages - any
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.