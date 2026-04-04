---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#8-summary
chunk_level: summary
chunk_type: prose
heading: TokenRequestSpec
token_count: 124
summary: * **boundObjectRef.apiVersion** (string) API version of the referent. * **boundObjectRef.kind** (string) Kind of the referent. Valid kinds are 'Pod' and 'Secret'. * **boundObjectRef.name** (string)...
---

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