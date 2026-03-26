---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#48-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 113
summary: # requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in # and that are required to be present in the service account. # The keys defined in this list...
---

# requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are required to be present in the service account.
# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest. If any of the keys defined in this list
# are not present in the service account, kubelet will not invoke the plugin and will return an error.
# This field is optional and may be empty. Plugins may use this field to extract additional information