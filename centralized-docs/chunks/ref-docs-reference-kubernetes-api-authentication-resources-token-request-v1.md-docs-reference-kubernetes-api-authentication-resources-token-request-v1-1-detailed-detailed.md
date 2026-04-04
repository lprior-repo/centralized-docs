---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#1-detailed
chunk_level: detailed
chunk_type: prose
heading: TokenRequestStatus
token_count: 957
summary: # TokenRequest TokenRequest requests a token for a given service account. `apiVersion: authentication.k8s.io/v1` `import \"k8s.io/api/authentication/v1\"` ## TokenRequest TokenRequest requests a token...
---

# TokenRequest
TokenRequest requests a token for a given service account.
`apiVersion: authentication.k8s.io/v1`
`import "k8s.io/api/authentication/v1"`
## TokenRequest
TokenRequest requests a token for a given service account.
* **apiVersion**: authentication.k8s.io/v1
* **kind**: TokenRequest
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **spec** ([TokenRequestSpec](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestSpec)), required
Spec holds information about the request being evaluated
* **status** ([TokenRequestStatus](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequestStatus))
Status is filled in by the server and indicates whether the token can be authenticated.
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
## TokenRequestStatus
TokenRequestStatus is the result of a token request.
* **expirationTimestamp** (Time), required
ExpirationTimestamp is the time of expiration of the returned token.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **token** (string), required
Token is the opaque bearer token.
#### Parameters
* **name** (*in path*): string, required
name of the TokenRequest
* **namespace** (*in path*): string, required
[namespace](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#namespace)
* **body**: [TokenRequest](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequest), required
* **dryRun** (*in query*): string
[dryRun](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#dryRun)
* **fieldManager** (*in query*): string
[fieldManager](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldManager)
* **fieldValidation** (*in query*): string
[fieldValidation](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#fieldValidation)
* **pretty** (*in query*): string
[pretty](https://kubernetes.io/docs/reference/kubernetes-api/common-parameters/common-parameters/#pretty)
#### Response
200 ([TokenRequest](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequest)): OK
201 ([TokenRequest](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequest)): Created
202 ([TokenRequest](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/#TokenRequest)): Accepted
401: Unauthorized