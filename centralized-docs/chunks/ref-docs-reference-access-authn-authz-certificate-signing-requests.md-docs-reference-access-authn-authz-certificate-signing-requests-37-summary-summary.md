---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#37-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 123
summary: 3. Permitted x509 extensions - honors subjectAltName and key usage extensions and discards other extensions. 4. Permitted key usages - must include `[\"client auth\"]`. Must not include key usages...
---

3. Permitted x509 extensions - honors subjectAltName and key usage extensions and discards other extensions.
4. Permitted key usages - must include `["client auth"]`. Must not include key usages beyond `["digital signature", "key encipherment", "client auth"]`.
5. Expiration/certificate lifetime - for the kube-controller-manager implementation of this signer, set to the minimum
of the `--cluster-signing-duration` option or, if specified, the `spec.expirationSeconds` field of the CSR object.
6. CA bit allowed/disallowed - not allowed.