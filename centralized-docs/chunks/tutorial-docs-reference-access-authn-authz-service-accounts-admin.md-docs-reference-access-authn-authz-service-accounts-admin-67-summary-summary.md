---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#67-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 54
summary: that lets containers authenticate as the right ServiceAccount. (This mechanism superseded an earlier mechanism that added a volume based on a Secret, where the Secret represented the ServiceAccount...
---

that lets containers authenticate as the right ServiceAccount.
(This mechanism superseded an earlier mechanism that added a volume based on a Secret,
where the Secret represented the ServiceAccount for the Pod but did not expire.)
Here's an example of how that looks for a launched Pod: