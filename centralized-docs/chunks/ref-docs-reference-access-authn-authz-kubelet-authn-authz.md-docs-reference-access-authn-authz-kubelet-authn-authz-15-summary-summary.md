---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#15-summary
chunk_level: summary
chunk_type: table
heading: Kubelet authentication
token_count: 127
summary: |*all others*|nodes|proxy| When the feature-gate `KubeletFineGrainedAuthz` is enabled, ensure the user identified by the `--kubelet-client-certificate` and `--kubelet-client-key` flags passed to the...
---

|*all others*|nodes|proxy|
When the feature-gate `KubeletFineGrainedAuthz` is enabled, ensure the user
identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
flags passed to the API server is authorized for the following attributes:
* verb=\*, resource=nodes, subresource=proxy
* verb=\*, resource=nodes, subresource=stats
* verb=\*, resource=nodes, subresource=log
* verb=\*, resource=nodes, subresource=metrics
* verb=\*, resource=nodes, subresource=configz