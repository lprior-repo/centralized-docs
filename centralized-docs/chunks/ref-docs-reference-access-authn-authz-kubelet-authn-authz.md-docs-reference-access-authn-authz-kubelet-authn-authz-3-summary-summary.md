---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#3-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 119
summary: * start the apiserver with `--kubelet-client-certificate` and `--kubelet-client-key` flags * see the [apiserver authentication...
---

* start the apiserver with `--kubelet-client-certificate` and `--kubelet-client-key` flags
* see the [apiserver authentication documentation](/docs/reference/access-authn-authz/authentication/#x509-client-certificates) for more details
To enable API bearer tokens (including service account tokens) to be used to authenticate to the kubelet's HTTPS endpoint:
* ensure the `authentication.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authentication-token-webhook` and `--kubeconfig` flags