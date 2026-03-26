---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#50-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 126
summary: # The keys defined in this list will be extracted from the corresponding service account and passed # to the plugin as part of the CredentialProviderRequest. The plugin is responsible for validating...
---

# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest. The plugin is responsible for validating the
# existence of annotations and their values. This field is optional and may be empty.
# Plugins may use this field to extract additional information required to fetch credentials.
# The keys defined in this list must be unique and not overlap with the keys defined in the
# +optional
optionalServiceAccountAnnotationKeys:
- "example.com/optional-annotation-key-1"
- "example.com/optional-annotation-key-2"
`
```