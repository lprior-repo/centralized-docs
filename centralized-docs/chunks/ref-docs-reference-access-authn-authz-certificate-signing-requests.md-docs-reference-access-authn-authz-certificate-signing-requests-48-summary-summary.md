---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#48-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 51
summary: None of these usages are related to ServiceAccount token secrets `.data[ca.crt]` in any way. That CA bundle is only guaranteed to verify a connection to the API server using the default service...
---

None of these usages are related to ServiceAccount token secrets `.data[ca.crt]` in any way. That CA bundle is only
guaranteed to verify a connection to the API server using the default service (`kubernetes.default.svc`).