---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#10-standard
chunk_level: standard
chunk_type: prose
heading: Authenticating service account credentials
token_count: 213
summary: ### Authenticating service account credentials in your own code If you have services of your own that need to validate Kubernetes service account credentials, you can use the following methods: *...
---

### Authenticating service account credentials in your own code
If you have services of your own that need to validate Kubernetes service
account credentials, you can use the following methods:
* [TokenReview API](/docs/reference/kubernetes-api/authentication-resources/token-review-v1/)
(recommended)
* OIDC discovery
The Kubernetes project recommends that you use the TokenReview API, because
this method invalidates tokens that are bound to API objects such as Secrets,
ServiceAccounts, Pods or Nodes when those objects are deleted. For example, if you
delete the Pod that contains a projected ServiceAccount token, the cluster
invalidates that token immediately and a TokenReview immediately fails.
If you use OIDC validation instead, your clients continue to treat the token
as valid until the token reaches its expiration timestamp.
Your application should always define the audience that it accepts, and should
check that the token's audiences match the audiences that the application
expects. This helps to minimize the scope of the token so that it can only be
used in your application and nowhere else.