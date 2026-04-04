---
doc_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests
chunk_id: ref/docs-reference-access-authn-authz-certificate-signing-requests.md/docs-reference-access-authn-authz-certificate-signing-requests#39-summary
chunk_level: summary
chunk_type: prose
heading: Signers
token_count: 88
summary: 2. Permitted subjects - organizations are exactly `[\"system:nodes\"]`, common name is \"`system:node:${NODE\_NAME}`\". 3. Permitted x509 extensions - honors key usage extensions, forbids subjectAltName...
---

2. Permitted subjects - organizations are exactly `["system:nodes"]`, common name is "`system:node:${NODE\_NAME}`".
3. Permitted x509 extensions - honors key usage extensions, forbids subjectAltName extensions and drops other extensions.
4. Permitted key usages - `["key encipherment", "digital signature", "client auth"]` or `["digital signature", "client auth"]`.