---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#2-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 78
summary: * start the kubelet with the `--anonymous-auth=false` flag To enable X509 client certificate authentication to the kubelet's HTTPS endpoint: * start the kubelet with the `--client-ca-file` flag,...
---

* start the kubelet with the `--anonymous-auth=false` flag
To enable X509 client certificate authentication to the kubelet's HTTPS endpoint:
* start the kubelet with the `--client-ca-file` flag, providing a CA bundle to verify client certificates with
* start the apiserver with `--kubelet-client-certificate` and `--kubelet-client-key` flags