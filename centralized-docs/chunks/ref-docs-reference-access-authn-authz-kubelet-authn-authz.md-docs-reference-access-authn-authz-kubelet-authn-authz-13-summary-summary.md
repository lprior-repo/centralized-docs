---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#13-summary
chunk_level: summary
chunk_type: table
heading: Kubelet authentication
token_count: 124
summary: * verb=\*, resource=nodes, subresource=metrics### Fine-grained authorization FEATURE STATE: `Kubernetes v1.33 [beta]`(enabled by default) When the feature gate `KubeletFineGrainedAuthz` is enabled...
---

* verb=\*, resource=nodes, subresource=metrics### Fine-grained authorization
FEATURE STATE:
`Kubernetes v1.33 [beta]`(enabled by default)
When the feature gate `KubeletFineGrainedAuthz` is enabled kubelet performs a
fine-grained check before falling back to the `proxy` subresource for the `/pods`,
`/runningPods`, `/configz` and `/healthz` endpoints. The resource and subresource
are determined from the incoming request's path:
|Kubelet API|resource|subresource|
|/stats/\