---
doc_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1
chunk_id: ref/docs-reference-kubernetes-api-authentication-resources-token-request-v1.md/docs-reference-kubernetes-api-authentication-resources-token-request-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: TokenRequest
token_count: 242
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