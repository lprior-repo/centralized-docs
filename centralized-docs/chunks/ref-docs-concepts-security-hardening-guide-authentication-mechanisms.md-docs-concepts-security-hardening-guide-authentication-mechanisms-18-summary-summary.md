---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#18-summary
chunk_level: summary
chunk_type: prose
heading: Authenticating proxy
token_count: 121
summary: Another option for integrating external authentication systems into Kubernetes is to use an [authenticating proxy](/docs/reference/access-authn-authz/authentication/#authenticating-proxy). With this...
---

Another option for integrating external authentication systems into Kubernetes is to use an
[authenticating proxy](/docs/reference/access-authn-authz/authentication/#authenticating-proxy).
With this mechanism, Kubernetes expects to receive requests from the proxy with specific header
values set, indicating the username and group memberships to assign for authorization purposes.
It is important to note that there are specific considerations to take into account when using
this mechanism.
Firstly, securely configured TLS must be used between the proxy and Kubernetes API server to
mitigate the risk of traffic interception or sniffing attacks. This ensures that the communication