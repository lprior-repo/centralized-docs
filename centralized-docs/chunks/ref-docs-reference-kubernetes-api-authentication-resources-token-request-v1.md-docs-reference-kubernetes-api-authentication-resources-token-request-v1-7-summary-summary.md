---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: TokenRequestSpec
token_count: 112
summary: * **boundObjectRef** (BoundObjectReference) BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The...
---

* **boundObjectRef** (BoundObjectReference)
BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
*BoundObjectReference is a reference to an object that a token is bound to.*
* **boundObjectRef.apiVersion** (string)
API version of the referent.