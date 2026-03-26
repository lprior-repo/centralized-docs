---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#49-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 123
summary: # This field is optional and may be empty. Plugins may use this field to extract additional information # required to fetch credentials or allow workloads to opt in to using service account tokens...
---

# This field is optional and may be empty. Plugins may use this field to extract additional information
# required to fetch credentials or allow workloads to opt in to using service account tokens for image pull.
# The keys defined in this list must be unique and not overlap with the keys defined in the
# optionalServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are optional to be present in the service account.
# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest.