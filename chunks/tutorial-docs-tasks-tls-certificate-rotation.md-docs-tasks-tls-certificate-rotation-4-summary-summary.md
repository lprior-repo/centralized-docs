---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#4-summary
chunk_level: summary
chunk_type: prose
heading: Enabling client certificate rotation
token_count: 90
summary: ## Enabling client certificate rotation The `kubelet` process accepts an argument `--rotate-certificates` that controls if the kubelet will automatically request a new certificate as the expiration...
---

## Enabling client certificate rotation
The `kubelet` process accepts an argument `--rotate-certificates` that controls
if the kubelet will automatically request a new certificate as the expiration of
the certificate currently in use approaches.
The `kube-controller-manager` process accepts an argument
`--cluster-signing-duration` (`--experimental-cluster-signing-duration` prior to 1.19)
that controls how long certificates will be issued for.