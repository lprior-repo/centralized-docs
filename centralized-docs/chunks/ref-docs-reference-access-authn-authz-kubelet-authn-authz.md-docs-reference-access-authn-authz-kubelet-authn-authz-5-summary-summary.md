---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#5-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 115
summary: * anonymous auth is enabled, but anonymous users' ability to call the kubelet API should be limited * bearer token auth is enabled, but arbitrary API users' (like service accounts) ability to call...
---

* anonymous auth is enabled, but anonymous users' ability to call the kubelet API should be limited
* bearer token auth is enabled, but arbitrary API users' (like service accounts) ability to call the kubelet API should be limited
* client certificate auth is enabled, but only some of the client certificates signed by the configured CA should be allowed to use the kubelet API
To subdivide access to the kubelet API, delegate authorization to the API server:
* ensure the `authorization.k8s.io/v1` API group is enabled in the API server