---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#56-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 96
summary: and not on pod-specific claims, then the plugin can set this to `ServiceAccount`. In this case, the kubelet will cache returned credentials on a per-service account basis. Use this when the returned...
---

and not on pod-specific claims,
then the plugin can set this to `ServiceAccount`.
In this case, the kubelet will cache returned credentials
on a per-service account basis.
Use this when the returned credential is valid for all pods using the same service account.
* `requireServiceAccount`:
whether the plugin requires the pod to have a service account.
* If set to `true`, kubelet will only invoke the plugin
if the pod has a service account.