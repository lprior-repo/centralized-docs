---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#55-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 121
summary: * `cacheType`: the type of cache key used for caching the credentials returned by the plugin when the service account token is used. The most conservative option is to set this to `Token`, which...
---

* `cacheType`:
the type of cache key used for caching the credentials returned by the plugin
when the service account token is used.
The most conservative option is to set this to `Token`,
which means the kubelet will cache returned credentials
on a per-token basis.
This should be set if the returned credential's lifetime
is limited to the service account token's lifetime.
If the plugin's credential retrieval logic depends only on the service account
and not on pod-specific claims,
then the plugin can set this to `ServiceAccount`.
In this case, the kubelet will cache returned credentials