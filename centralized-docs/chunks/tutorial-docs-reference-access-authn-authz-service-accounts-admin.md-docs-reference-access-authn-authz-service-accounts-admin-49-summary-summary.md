---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#49-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 112
summary: 3. update the Secret label `kubernetes.io/legacy-token-last-used` with the new date, 4. return an error indicating that the token has been invalidated. When receiving this validation error, users can...
---

3. update the Secret label `kubernetes.io/legacy-token-last-used` with the new
date,
4. return an error indicating that the token has been invalidated.
When receiving this validation error, users can update the Secret to remove the
`kubernetes.io/legacy-token-invalid-since` label to temporarily allow use of
this token.
Here's an example of an auto-generated legacy token that has been marked with the
`kubernetes.io/legacy-token-last-used` and `kubernetes.io/legacy-token-invalid-since`
labels: