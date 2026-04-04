---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#95-summary
chunk_level: summary
chunk_type: prose
heading: External ServiceAccount token signing and key management
token_count: 39
summary: ### Sign Sign takes a serialized JWT payload, and returns the serialized header and signature. `kube-apiserver` then assembles the JWT from the header, payload, and signature.
---

### Sign
Sign takes a serialized JWT payload, and returns the serialized header and
signature. `kube-apiserver` then assembles the JWT from the header, payload,
and signature.