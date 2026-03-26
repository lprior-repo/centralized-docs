---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#1-standard
chunk_level: standard
chunk_type: prose
heading: Enabling client certificate rotation
token_count: 262
summary: # Configure Certificate Rotation for the Kubelet This page shows how to enable and configure certificate rotation for the kubelet. FEATURE STATE: `Kubernetes v1.19 [stable]` ## Before you begin *...
---

# Configure Certificate Rotation for the Kubelet
This page shows how to enable and configure certificate rotation for the kubelet.
FEATURE STATE:
`Kubernetes v1.19 [stable]`
## Before you begin
* Kubernetes version 1.8.0 or later is required## Overview
The kubelet uses certificates for authenticating to the Kubernetes API. By
default, these certificates are issued with one year expiration so that they do
not need to be renewed too frequently.
Kubernetes contains [kubelet certificate
rotation](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/),
that will automatically generate a new key and request a new certificate from
the Kubernetes API as the current certificate approaches expiration. Once the
new certificate is available, it will be used for authenticating connections to
the Kubernetes API.
## Enabling client certificate rotation
The `kubelet` process accepts an argument `--rotate-certificates` that controls
if the kubelet will automatically request a new certificate as the expiration of
the certificate currently in use approaches.
The `kube-controller-manager` process accepts an argument
`--cluster-signing-duration` (`--experimental-cluster-signing-duration` prior to 1.19)
that controls how long certificates will be issued for.