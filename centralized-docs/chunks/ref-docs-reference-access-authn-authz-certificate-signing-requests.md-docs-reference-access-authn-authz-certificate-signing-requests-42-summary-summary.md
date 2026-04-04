---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#42-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 97
summary: 4. Permitted key usages - `[\"key encipherment\", \"digital signature\", \"server auth\"]` or `[\"digital signature\", \"server auth\"]`. 5. Expiration/certificate lifetime - for the kube-controller-manager...
---

4. Permitted key usages - `["key encipherment", "digital signature", "server auth"]` or `["digital signature", "server auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.