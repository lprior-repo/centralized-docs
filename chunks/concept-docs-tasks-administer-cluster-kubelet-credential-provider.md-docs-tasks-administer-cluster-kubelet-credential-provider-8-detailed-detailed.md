---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Configuring the Kubelet
token_count: 962
summary: # pod-specific claims, then the plugin can set this to \"ServiceAccount\". In this case, the # kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the # returned...
---

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
* `name`: the name of the plugin which MUST match the name of the executable binary that exists
in the directory passed into `--image-credential-provider-bin-dir`.
* `matchImages`: a list of strings used to match against images in order to determine
if this provider should be invoked. More on this below.
* `defaultCacheDuration`: the default duration the kubelet will cache credentials in-memory
if a cache duration was not specified by the plugin.
* `apiVersion`: the API version that the kubelet and the exec plugin will use when communicating.
Each credential provider can also be given optional args and environment variables as well.
Consult the plugin implementors to determine what set of arguments and environment variables are required for a given plugin.
If you are using the KubeletServiceAccountTokenForCredentialProviders feature gate
and configuring the plugin to use the service account token
by setting the tokenAttributes field,
the following fields are required:
* `serviceAccountTokenAudience`:
the intended audience for the projected service account token.
This cannot be the empty string.
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
on a per-service account basis.
Use this when the returned credential is valid for all pods using the same service account.
* `requireServiceAccount`:
whether the plugin requires the pod to have a service account.
* If set to `true`, kubelet will only invoke the plugin
if the pod has a service account.
* If set to `false`, kubelet will invoke the plugin
even if the pod does not have a service account
and will not include a token in the `CredentialProviderRequest`.
This is useful for plugins that are used
to pull images for pods without service accounts
(e.g., static pods).