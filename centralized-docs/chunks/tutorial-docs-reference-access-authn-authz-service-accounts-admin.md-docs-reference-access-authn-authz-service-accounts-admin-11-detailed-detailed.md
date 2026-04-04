---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#11-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 602
summary: ### Sign Sign takes a serialized JWT payload, and returns the serialized header and signature. `kube-apiserver` then assembles the JWT from the header, payload, and signature. ``` `rpc...
---

### Sign
Sign takes a serialized JWT payload, and returns the serialized header and
signature. `kube-apiserver` then assembles the JWT from the header, payload,
and signature.
```
`rpc Sign(SignJWTRequest) returns (SignJWTResponse) {}
message SignJWTRequest {
// URL-safe base64 wrapped payload to be signed.
// Exactly as it appears in the second segment of the JWT
string claims = 1;
}
message SignJWTResponse {
// header must contain only alg, kid, typ claims.
// typ must be “JWT”.
// kid must be non-empty, &lt;=1024 characters, and its corresponding public key should not be excluded from OIDC discovery.
// alg must be one of the algorithms supported by kube-apiserver (currently RS256, ES256, ES384, ES512).
// header cannot have any additional data that kube-apiserver does not recognize.
// Already wrapped in URL-safe base64, exactly as it appears in the first segment of the JWT.
string header = 1;
// The signature for the JWT.
// Already wrapped in URL-safe base64, exactly as it appears in the final segment of the JWT.
string signature = 2;
}
`
```
## Clean up
If you created a namespace `examplens` to experiment with, you can remove it:
```
`kubectl delete namespace examplens
`
```
## What's next
* Read more details about [projected volumes](/docs/concepts/storage/projected-volumes/).
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified July 29, 2025 at 2:30 PM PST: [KEP-740: move grpc docs into website (c6c320d7d4)](https://github.com/kubernetes/website/commit/c6c320d7d49beea6deb57f650d92c4562dc06bc4)
## Related Pages

- [Controlling Access to the Kubernetes API](docs-concepts-security-controlling-access.md)
- [Volumes](docs-concepts-storage-volumes.md)
- [Концепции](ru-docs-concepts.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)