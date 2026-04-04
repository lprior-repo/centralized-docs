---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#7-summary
chunk_level: summary
chunk_type: table
heading: Kubelet authentication
token_count: 122
summary: * the kubelet calls the `SubjectAccessReview` API on the configured API server to determine whether each request is authorized The kubelet authorizes API requests using the same [request...
---

* the kubelet calls the `SubjectAccessReview` API on the configured API server to determine whether each request is authorized
The kubelet authorizes API requests using the same [request attributes](/docs/reference/access-authn-authz/authorization/#review-your-request-attributes) approach as the apiserver.
The verb is determined from the incoming request's HTTP verb:
|HTTP verb|request verb|
|POST|create|
|GET, HEAD|get|
|PUT|update|
|PATCH|patch|
|DELETE|delete|
The resource and subresource is determined from the incoming request's path: