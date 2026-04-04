---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#6-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 44
summary: * ensure the `authorization.k8s.io/v1` API group is enabled in the API server * start the kubelet with the `--authorization-mode=Webhook` and the `--kubeconfig` flags
---

* ensure the `authorization.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authorization-mode=Webhook` and the `--kubeconfig` flags