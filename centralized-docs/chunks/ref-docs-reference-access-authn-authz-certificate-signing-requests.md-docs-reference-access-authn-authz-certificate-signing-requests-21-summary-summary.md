---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#21-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 70
summary: verbs: - update - apiGroups: - certificates.k8s.io resources: - signers resourceNames: - example.com/my-signer-name # example.com/\* can be used to authorize for all signers in the 'example.com'...
---

verbs:
- update
- apiGroups:
- certificates.k8s.io
resources:
- signers
resourceNames:
- example.com/my-signer-name # example.com/\* can be used to authorize for all signers in the 'example.com' domain
verbs:
- approve
`
```
To allow signing a CertificateSigningRequest: