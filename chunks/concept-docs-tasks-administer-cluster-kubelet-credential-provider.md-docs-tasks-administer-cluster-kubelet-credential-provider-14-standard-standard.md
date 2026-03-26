---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#14-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the Kubelet
token_count: 511
summary: # If the plugin's credential retrieval logic depends only on the service account and not on # pod-specific claims, then the plugin can set this to \"ServiceAccount\". In this case, the # kubelet will...
---

# If the plugin's credential retrieval logic depends only on the service account and not on
# pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the
# kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the
# returned credential is valid for all pods using the same service account.
# requireServiceAccount indicates whether the plugin requires the pod to have a service account.
# If set to true, kubelet will only invoke the plugin if the pod has a service account.
# If set to false, kubelet will invoke the plugin even if the pod does not have a service account
# and will not include a token in the CredentialProviderRequest. This is useful for plugins
# that are used to pull images for pods without service accounts (e.g., static pods).
# requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are required to be present in the service account.
# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest. If any of the keys defined in this list
# are not present in the service account, kubelet will not invoke the plugin and will return an error.
# This field is optional and may be empty. Plugins may use this field to extract additional information
# required to fetch credentials or allow workloads to opt in to using service account tokens for image pull.
# The keys defined in this list must be unique and not overlap with the keys defined in the
# optionalServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are optional to be present in the service account.
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
The `providers` field is a list of enabled plugins used by the kubelet. Each entry has a few required fields: