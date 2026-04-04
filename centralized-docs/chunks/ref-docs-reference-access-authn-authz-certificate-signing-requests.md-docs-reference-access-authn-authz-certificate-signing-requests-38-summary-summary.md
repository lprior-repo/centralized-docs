---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#38-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 125
summary: 6. CA bit allowed/disallowed - not allowed. 7. `kubernetes.io/kube-apiserver-client-kubelet`: signs client certificates that will be honored as client certificates by the API server. May be...
---

6. CA bit allowed/disallowed - not allowed.
7. `kubernetes.io/kube-apiserver-client-kubelet`: signs client certificates that will be honored as client certificates by the
API server.
May be auto-approved by [kube-controller-manager](/docs/reference/command-line-tools-reference/kube-controller-manager/).
1. Trust distribution: signed certificates must be honored as client certificates by the API server. The CA bundle
is not distributed by any other means.
2. Permitted subjects - organizations are exactly `["system:nodes"]`, common name is "`system:node:${NODE\_NAME}`".