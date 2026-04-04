---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#35-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 78
summary: 1. `kubernetes.io/kube-apiserver-client`: signs certificates that will be honored as client certificates by the API server. Never auto-approved by...
---

1. `kubernetes.io/kube-apiserver-client`: signs certificates that will be honored as client certificates by the API server.
Never auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle is not distributed by any other means.