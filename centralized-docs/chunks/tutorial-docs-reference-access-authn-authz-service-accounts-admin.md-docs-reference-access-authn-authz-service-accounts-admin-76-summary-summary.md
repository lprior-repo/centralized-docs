---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#76-summary
chunk_level: summary
chunk_type: prose
heading: Auto-generated legacy ServiceAccount token clean up
token_count: 99
summary: If you launch a new Pod into the `examplens` namespace, it can use the `myserviceaccount` service-account-token Secret that you just created. #### Caution: Do not reference manually created Secrets...
---

If you launch a new Pod into the `examplens` namespace, it can use the `myserviceaccount`
service-account-token Secret that you just created.
#### Caution:
Do not reference manually created Secrets in the `secrets` field of a
ServiceAccount. Or the manually created Secrets will be cleaned if it is not used for a long
time. Please refer to [auto-generated legacy ServiceAccount token clean up](#auto-generated-legacy-serviceaccount-token-clean-up).