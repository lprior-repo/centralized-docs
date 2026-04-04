---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#3-standard
chunk_level: standard
chunk_type: table
heading: Kubelet authentication
token_count: 284
summary: * client certificate auth is enabled, but only some of the client certificates signed by the configured CA should be allowed to use the kubelet API To subdivide access to the kubelet API, delegate...
---

* client certificate auth is enabled, but only some of the client certificates signed by the configured CA should be allowed to use the kubelet API
To subdivide access to the kubelet API, delegate authorization to the API server:
* ensure the `authorization.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authorization-mode=Webhook` and the `--kubeconfig` flags
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
|Kubelet API|resource|subresource|
|/stats/\*|nodes|stats|
|/metrics/\*|nodes|metrics|
|/logs/\*|nodes|log|
|/spec/\*|nodes|spec|
|/checkpoint/\*|nodes|checkpoint|
|*all others*|nodes|proxy|