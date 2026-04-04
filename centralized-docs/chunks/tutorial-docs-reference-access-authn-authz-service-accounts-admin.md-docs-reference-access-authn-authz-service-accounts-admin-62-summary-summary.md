---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#62-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 117
summary: * The Secret is auto-generated, meaning that it is bi-directionally referenced by a ServiceAccount. * The Secret is not currently mounted by any pods. * The Secret has not been used in a *specified...
---

* The Secret is auto-generated, meaning that it is bi-directionally referenced
by a ServiceAccount.
* The Secret is not currently mounted by any pods.
* The Secret has not been used in a *specified amount of time* since it was
created or since it was last used.
The cleaner marks a Secret invalid by adding a label called
`kubernetes.io/legacy-token-invalid-since` to the Secret, with the current date
as the value. If an invalid Secret is not used in a *specified amount of time*,
the cleaner will delete it.