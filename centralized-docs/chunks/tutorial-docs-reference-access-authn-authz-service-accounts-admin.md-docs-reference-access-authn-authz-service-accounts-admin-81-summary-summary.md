---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#81-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 97
summary: ### Delete/invalidate a short-lived ServiceAccount token Short lived ServiceAccount tokens automatically expire after the time-limit specified during their creation. There is no central record of...
---

### Delete/invalidate a short-lived ServiceAccount token
Short lived ServiceAccount tokens automatically expire after the time-limit
specified during their creation. There is no central record of tokens issued,
so there is no way to revoke individual tokens.
If you have to revoke a short-lived token before its expiration, you
can delete and re-create the ServiceAccount it is associated to. This will
change its UID and hence invalidate **all** ServiceAccount tokens that were
created for it.