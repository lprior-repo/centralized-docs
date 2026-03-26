---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#33-summary
chunk_level: summary
chunk_type: prose
heading: Service Account Token for Image Pulls
token_count: 105
summary: * If you are configuring a credential provider plugin that requires the service account token, you need a Kubernetes cluster with nodes running Kubernetes v1.33 or later and the...
---

* If you are configuring a credential provider plugin
that requires the service account token,
you need a Kubernetes cluster with nodes running Kubernetes v1.33 or later
and the `KubeletServiceAccountTokenForCredentialProviders` feature gate
enabled on the kubelet.
* A working implementation of a credential provider exec plugin. You can build your own plugin or use one provided by cloud providers.Your Kubernetes server must be at or later than version v1.26.
To check the version, enter `kubectl version`.