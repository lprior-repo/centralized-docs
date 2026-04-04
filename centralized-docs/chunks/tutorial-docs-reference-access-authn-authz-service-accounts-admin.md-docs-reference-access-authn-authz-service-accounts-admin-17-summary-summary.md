---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#17-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 55
summary: then for any instant that is 60 seconds (or more) after the `.metadata.deletionTimestamp` date, authentication with that token would fail. If the referenced object no longer exists (or its...
---

then for any instant that is 60 seconds (or more) after the `.metadata.deletionTimestamp` date,
authentication with that token would fail.
If the referenced object no longer exists (or its `metadata.uid` does not match),
the request will not be authenticated.