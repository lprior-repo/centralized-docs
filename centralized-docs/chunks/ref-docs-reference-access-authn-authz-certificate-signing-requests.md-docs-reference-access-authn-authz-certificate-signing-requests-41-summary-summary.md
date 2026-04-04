---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#41-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 114
summary: 1. Trust distribution: signed certificates must be honored by the API server as valid to terminate connections to a kubelet. The CA bundle is not distributed by any other means. 2. Permitted subjects...
---

1. Trust distribution: signed certificates must be honored by the API server as valid to terminate connections to a kubelet.
The CA bundle is not distributed by any other means.
2. Permitted subjects - organizations are exactly `["system:nodes"]`, common name is "`system:node:${NODE\_NAME}`".
3. Permitted x509 extensions - honors key usage and DNSName/IPAddress subjectAltName extensions, forbids EmailAddress and
URI subjectAltName extensions, drops other extensions. At least one DNS or IP subjectAltName must be present.