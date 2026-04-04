---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#2-standard
chunk_level: standard
chunk_type: prose
heading: TokenRequestSpec
token_count: 323
summary: ## TokenRequestSpec TokenRequestSpec contains client provided parameters of a token request. * **audiences** ([]string), required *Atomic: will be replaced during a merge* Audiences are the intendend...
---

## TokenRequestSpec
TokenRequestSpec contains client provided parameters of a token request.
* **audiences** ([]string), required
*Atomic: will be replaced during a merge*
Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.
* **boundObjectRef** (BoundObjectReference)
BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
*BoundObjectReference is a reference to an object that a token is bound to.*
* **boundObjectRef.apiVersion** (string)
API version of the referent.
* **boundObjectRef.kind** (string)
Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
* **boundObjectRef.name** (string)
Name of the referent.
* **boundObjectRef.uid** (string)
UID of the referent.
* **expirationSeconds** (int64)
ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response.