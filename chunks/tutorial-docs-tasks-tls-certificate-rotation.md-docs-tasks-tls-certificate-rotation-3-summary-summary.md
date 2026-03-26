---
doc_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation
chunk_id: tutorial/docs-tasks-tls-certificate-rotation.md/docs-tasks-tls-certificate-rotation#3-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 128
summary: * Kubernetes version 1.8.0 or later is required## Overview The kubelet uses certificates for authenticating to the Kubernetes API. By default, these certificates are issued with one year expiration...
---

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