---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#8-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 210
summary: # Configure a kubelet image credential provider FEATURE STATE: `Kubernetes v1.26 [stable]` Starting from Kubernetes v1.20, the kubelet can dynamically retrieve credentials for a container image...
---

# Configure a kubelet image credential provider
FEATURE STATE:
`Kubernetes v1.26 [stable]`
Starting from Kubernetes v1.20, the kubelet can dynamically retrieve credentials for a container image registry
using exec plugins. The kubelet and the exec plugin communicate through stdio (stdin, stdout, and stderr) using
Kubernetes versioned APIs. These plugins allow the kubelet to request credentials for a container registry dynamically
as opposed to storing static credentials on disk. For example, the plugin may talk to a local metadata server to retrieve
short-lived credentials for an image that is being pulled by the kubelet.
You may be interested in using this capability if any of the below are true:
* API calls to a cloud provider service are required to retrieve authentication information for a registry.
* Credentials have short expiration times and requesting new credentials frequently is required.
* Storing registry credentials on disk or in imagePullSecrets is not acceptable.
This guide demonstrates how to configure the kubelet's image credential provider plugin mechanism.