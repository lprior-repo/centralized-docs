---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#44-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 126
summary: The returned CredentialProviderResponse # MUST use the same encoding version as the input. Current supported values are: # Arguments to pass to the command when executing it. # Env defines additional...
---

The returned CredentialProviderResponse
# MUST use the same encoding version as the input. Current supported values are:
# Arguments to pass to the command when executing it.
# Env defines additional environment variables to expose to the process. These
# are unioned with the host's environment, as well as variables client-go uses
# tokenAttributes is the configuration for the service account token that will be passed to the plugin.
# The credential provider opts in to using service account tokens for image pull by setting this field.
# if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,