---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#16-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 128
summary: * Node (can be used to auto-revoke a token when its Node is deleted; creating new node-bound tokens is GA in v1.33+) When a token is bound to an object, the object's `metadata.name` and...
---

* Node (can be used to auto-revoke a token when its Node is deleted; creating new node-bound tokens is GA in v1.33+)
When a token is bound to an object, the object's `metadata.name` and `metadata.uid` are
stored as extra 'private claims' in the issued JWT.
When a bound token is presented to the kube-apiserver, the service account authenticator
will extract and verify these claims.
If the referenced object or the ServiceAccount is pending deletion (for example, due to finalizers),
then for any instant that is 60 seconds (or more) after the