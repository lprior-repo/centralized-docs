---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#26-summary
chunk_level: summary
chunk_type: prose
heading: Certificate signing requests
token_count: 61
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
- sign
`
```