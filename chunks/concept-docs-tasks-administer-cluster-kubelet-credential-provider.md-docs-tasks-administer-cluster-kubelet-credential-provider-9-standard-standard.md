---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#9-standard
chunk_level: standard
chunk_type: prose
heading: Installing Plugins on Nodes
token_count: 444
summary: ## Service Account Token for Image Pulls FEATURE STATE: `Kubernetes v1.34 [beta]`(enabled by default) Starting from Kubernetes v1.33, the kubelet can be configured to send a service account token...
---

## Service Account Token for Image Pulls
FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
Starting from Kubernetes v1.33,
the kubelet can be configured to send a service account token
bound to the pod for which the image pull is being performed
to the credential provider plugin.
This allows the plugin to exchange the token for credentials
to access the image registry.
To enable this feature,
the `KubeletServiceAccountTokenForCredentialProviders` feature gate
must be enabled on the kubelet,
and the `tokenAttributes` field must be set
in the `CredentialProviderConfig` file for the plugin.
The `tokenAttributes` field contains information
about the service account token that will be passed to the plugin,
including the intended audience for the token
and whether the plugin requires the pod to have a service account.
Using service account token credentials can enable the following use-cases:
* Avoid needing a kubelet/node-based identity to pull images from a registry.
* Allow workloads to pull images based on their own runtime identity
without long-lived/persisted secrets.## Before you begin
* You need a Kubernetes cluster with nodes that support kubelet credential
provider plugins. This support is available in Kubernetes 1.35;
Kubernetes v1.24 and v1.25 included this as a beta feature, enabled by default.
* If you are configuring a credential provider plugin
that requires the service account token,
you need a Kubernetes cluster with nodes running Kubernetes v1.33 or later
and the `KubeletServiceAccountTokenForCredentialProviders` feature gate
enabled on the kubelet.
* A working implementation of a credential provider exec plugin. You can build your own plugin or use one provided by cloud providers.Your Kubernetes server must be at or later than version v1.26.
To check the version, enter `kubectl version`.
## Installing Plugins on Nodes
A credential provider plugin is an executable binary that will be run by the kubelet. Ensure that the plugin binary exists on
every node in your cluster and stored in a known directory. The directory will be required later when configuring kubelet flags.