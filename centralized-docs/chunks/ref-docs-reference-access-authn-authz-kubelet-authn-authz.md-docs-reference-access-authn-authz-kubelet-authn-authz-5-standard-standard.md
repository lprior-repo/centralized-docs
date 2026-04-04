---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#5-standard
chunk_level: standard
chunk_type: table
heading: Kubelet authentication
token_count: 459
summary: * verb=\*, resource=nodes, subresource=proxy * verb=\*, resource=nodes, subresource=stats * verb=\*, resource=nodes, subresource=log * verb=\*, resource=nodes, subresource=spec * verb=\*,...
---

* verb=\*, resource=nodes, subresource=proxy
* verb=\*, resource=nodes, subresource=stats
* verb=\*, resource=nodes, subresource=log
* verb=\*, resource=nodes, subresource=spec
* verb=\*, resource=nodes, subresource=metrics### Fine-grained authorization
FEATURE STATE:
`Kubernetes v1.33 [beta]`(enabled by default)
When the feature gate `KubeletFineGrainedAuthz` is enabled kubelet performs a
fine-grained check before falling back to the `proxy` subresource for the `/pods`,
`/runningPods`, `/configz` and `/healthz` endpoints. The resource and subresource
are determined from the incoming request's path:
|Kubelet API|resource|subresource|
|/stats/\*|nodes|stats|
|/metrics/\*|nodes|metrics|
|/logs/\*|nodes|log|
|/pods|nodes|pods, proxy|
|/runningPods/|nodes|pods, proxy|
|/healthz|nodes|healthz, proxy|
|/configz|nodes|configz, proxy|
|*all others*|nodes|proxy|
When the feature-gate `KubeletFineGrainedAuthz` is enabled, ensure the user
identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
flags passed to the API server is authorized for the following attributes:
* verb=\*, resource=nodes, subresource=proxy
* verb=\*, resource=nodes, subresource=stats
* verb=\*, resource=nodes, subresource=log
* verb=\*, resource=nodes, subresource=metrics
* verb=\*, resource=nodes, subresource=configz
* verb=\*, resource=nodes, subresource=healthz
* verb=\*, resource=nodes, subresource=pods
If [RBAC authorization](/docs/reference/access-authn-authz/rbac/) is used,
enabling this gate also ensure that the builtin `system:kubelet-api-admin` ClusterRole
is updated with permissions to access all the above mentioned subresources.