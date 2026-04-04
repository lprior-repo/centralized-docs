---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#40-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 120
summary: 5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum of the `--cluster-signing-duration` option or, if specified, the...
---

5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/kubelet-serving`: signs serving certificates that are honored as a valid kubelet serving certificate
by the API server, but has no other guarantees.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).