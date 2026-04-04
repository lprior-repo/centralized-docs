---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#48-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 80
summary: 1. add an audit annotation for the key-value pair `authentication.k8s.io/legacy-token-invalidated: &lt;secret name&gt;/&lt;namespace&gt;`, 2. increment the `invalid\_legacy\_auto\_token\_uses\_total`...
---

1. add an audit annotation for the key-value pair
`authentication.k8s.io/legacy-token-invalidated: &lt;secret name&gt;/&lt;namespace&gt;`,
2. increment the `invalid\_legacy\_auto\_token\_uses\_total` metric count,
3. update the Secret label `kubernetes.io/legacy-token-last-used` with the new
date,