---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#4-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 116
summary: * start the kubelet with the `--authentication-token-webhook` and `--kubeconfig` flags * the kubelet calls the `TokenReview` API on the configured API server to determine user information from bearer...
---

* start the kubelet with the `--authentication-token-webhook` and `--kubeconfig` flags
* the kubelet calls the `TokenReview` API on the configured API server to determine user information from bearer tokens## Kubelet authorization
Any request that is successfully authenticated (including an anonymous request) is then authorized. The default authorization mode is `AlwaysAllow`, which allows all requests.
There are many possible reasons to subdivide access to the kubelet API:
* anonymous auth is enabled, but anonymous users' ability to call the kubelet API should be limited