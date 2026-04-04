---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#52-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 44
summary: * watches for ServiceAccount deletion and deletes all corresponding ServiceAccount token Secrets. * watches for ServiceAccount token Secret addition, and ensures the referenced ServiceAccount exists,...
---

* watches for ServiceAccount deletion and deletes all corresponding ServiceAccount
token Secrets.
* watches for ServiceAccount token Secret addition, and ensures the referenced
ServiceAccount exists, and adds a token to the Secret if needed.