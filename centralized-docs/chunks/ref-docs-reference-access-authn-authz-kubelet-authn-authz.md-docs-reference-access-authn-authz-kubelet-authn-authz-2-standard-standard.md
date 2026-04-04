---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#2-standard
chunk_level: standard
chunk_type: prose
heading: Kubelet authentication
token_count: 388
summary: * start the kubelet with the `--anonymous-auth=false` flag To enable X509 client certificate authentication to the kubelet's HTTPS endpoint: * start the kubelet with the `--client-ca-file` flag,...
---

* start the kubelet with the `--anonymous-auth=false` flag
To enable X509 client certificate authentication to the kubelet's HTTPS endpoint:
* start the kubelet with the `--client-ca-file` flag, providing a CA bundle to verify client certificates with
* start the apiserver with `--kubelet-client-certificate` and `--kubelet-client-key` flags
* see the [apiserver authentication documentation](/docs/reference/access-authn-authz/authentication/#x509-client-certificates) for more details
To enable API bearer tokens (including service account tokens) to be used to authenticate to the kubelet's HTTPS endpoint:
* ensure the `authentication.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authentication-token-webhook` and `--kubeconfig` flags
* the kubelet calls the `TokenReview` API on the configured API server to determine user information from bearer tokens## Kubelet authorization
Any request that is successfully authenticated (including an anonymous request) is then authorized. The default authorization mode is `AlwaysAllow`, which allows all requests.
There are many possible reasons to subdivide access to the kubelet API:
* anonymous auth is enabled, but anonymous users' ability to call the kubelet API should be limited
* bearer token auth is enabled, but arbitrary API users' (like service accounts) ability to call the kubelet API should be limited
* client certificate auth is enabled, but only some of the client certificates signed by the configured CA should be allowed to use the kubelet API
To subdivide access to the kubelet API, delegate authorization to the API server:
* ensure the `authorization.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authorization-mode=Webhook` and the `--kubeconfig` flags