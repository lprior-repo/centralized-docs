---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#2-standard
chunk_level: standard
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 78
summary: ## X.509 client certificate authentication Kubernetes leverages [X.509 client certificate](/docs/reference/access-authn-authz/authentication/#x509-client-certificates) authentication for system...
---

## X.509 client certificate authentication
Kubernetes leverages [X.509 client certificate](/docs/reference/access-authn-authz/authentication/#x509-client-certificates)
authentication for system components, such as when the kubelet authenticates to the API Server.
While this mechanism can also be used for user authentication, it might not be suitable for
production use due to several restrictions: