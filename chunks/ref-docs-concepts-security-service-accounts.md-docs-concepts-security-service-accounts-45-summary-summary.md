---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#45-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating service account credentials
token_count: 103
summary: `TokenRequest` API, the API server also checks that the specific object reference that is using the ServiceAccount still exists, matching by the [unique...
---

`TokenRequest` API, the API server also checks that
the specific object reference that is using the ServiceAccount still exists,
matching by the [unique ID](/docs/concepts/overview/working-with-objects/names) of that
object. For legacy tokens that are mounted as Secrets in Pods, the API server
checks the token against the Secret.
For more information about the authentication process, refer to
[Authentication](/docs/reference/access-authn-authz/authentication/#service-account-tokens).