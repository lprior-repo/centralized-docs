---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#2-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 936
summary: #### Warning: `nodes/proxy` permission grants access to all other kubelet APIs. This includes APIs that can be used to execute commands in any container running on the node. Some of these endpoints...
---

#### Warning:
`nodes/proxy` permission grants access to all other kubelet APIs.
This includes APIs that can be used to execute commands in any container running on the node.
Some of these endpoints support Websocket protocols via HTTP `GET` requests, which are authorized with the **get** verb.
This means that **get** permission on `nodes/proxy` is not a read-only permission,
and authorizes executing commands in any container running on the node.
The namespace and API group attributes are always an empty string, and
the resource name is always the name of the kubelet's `Node` API object.
When running in this mode, ensure the user identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
flags passed to the apiserver is authorized for the following attributes:
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified January 28, 2026 at 10:04 AM PST: [Strengthen and clarify nodes/proxy warnings (b78e1d54b8)](https://github.com/kubernetes/website/commit/b78e1d54b8caee2f7b3433386ad9d7a8ebefeb25)
## Related Pages

- [Certificates and Certificate Signing Requests](docs-reference-access-authn-authz-certificate-signing-requests.md)
- [Controlling Access to the Kubernetes API](docs-concepts-security-controlling-access.md)
- [Hardening Guide - Authentication Mechanisms](docs-concepts-security-hardening-guide-authentication-mechanisms.md)
- [Securing a Cluster](docs-tasks-administer-cluster-securing-a-cluster.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)