---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#4-summary
chunk_level: summary
chunk_type: prose
heading: TokenRequest
token_count: 85
summary: * **spec** ([TokenRequestSpec](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestSpec)), required Spec holds information about the request...
---

* **spec** ([TokenRequestSpec](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestSpec)), required
Spec holds information about the request being evaluated
* **status** ([TokenRequestStatus](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestStatus))
Status is filled in by the server and indicates whether the token can be authenticated.