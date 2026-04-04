---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#9-summary
chunk_level: summary
chunk_type: prose
heading: TokenRequestStatus
token_count: 88
summary: ## TokenRequestStatus TokenRequestStatus is the result of a token request. * **expirationTimestamp** (Time), required ExpirationTimestamp is the time of expiration of the returned token. *Time is a...
---

## TokenRequestStatus
TokenRequestStatus is the result of a token request.
* **expirationTimestamp** (Time), required
ExpirationTimestamp is the time of expiration of the returned token.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **token** (string), required
Token is the opaque bearer token.