---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#39-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 124
summary: providers: # name is the required name of the credential provider. It must match the name of the # provider executable as seen by the kubelet. The executable must be in the kubelet's # bin directory...
---

providers:
# name is the required name of the credential provider. It must match the name of the
# provider executable as seen by the kubelet. The executable must be in the kubelet's
# bin directory (set by the --image-credential-provider-bin-dir flag).
- name: ecr-credential-provider
# matchImages is a required list of strings used to match against images in order to
# determine if this provider should be invoked. If one of the strings matches the
# requested image from the kubelet, the plugin will be invoked and given a chance
# to provide credentials.