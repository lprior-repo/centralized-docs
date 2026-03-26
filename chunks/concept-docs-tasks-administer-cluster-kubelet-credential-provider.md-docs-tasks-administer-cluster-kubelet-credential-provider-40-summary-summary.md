---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#40-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 110
summary: # requested image from the kubelet, the plugin will be invoked and given a chance # to provide credentials. Images are expected to contain the registry domain # Each entry in matchImages is a pattern...
---

# requested image from the kubelet, the plugin will be invoked and given a chance
# to provide credentials. Images are expected to contain the registry domain
# Each entry in matchImages is a pattern which can optionally contain a port and a path.
# Globs can be used in the domain, but not in the port or the path. Globs are supported
# as subdomains like '\*.k8s.io' or 'k8s.\*.io', and top-level-domains such as 'k8s.\*'.