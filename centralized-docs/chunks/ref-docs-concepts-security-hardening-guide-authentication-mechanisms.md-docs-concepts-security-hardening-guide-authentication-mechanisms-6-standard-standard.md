---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#6-standard
chunk_level: standard
chunk_type: prose
heading: Feedback
token_count: 490
summary: ## Authenticating proxy Another option for integrating external authentication systems into Kubernetes is to use an [authenticating...
---

## Authenticating proxy
Another option for integrating external authentication systems into Kubernetes is to use an
[authenticating proxy](/docs/reference/access-authn-authz/authentication/#authenticating-proxy).
With this mechanism, Kubernetes expects to receive requests from the proxy with specific header
values set, indicating the username and group memberships to assign for authorization purposes.
It is important to note that there are specific considerations to take into account when using
this mechanism.
Firstly, securely configured TLS must be used between the proxy and Kubernetes API server to
mitigate the risk of traffic interception or sniffing attacks. This ensures that the communication
between the proxy and Kubernetes API server is secure.
Secondly, it is important to be aware that an attacker who is able to modify the headers of the
request may be able to gain unauthorized access to Kubernetes resources. As such, it is important
to ensure that the headers are properly secured and cannot be tampered with.
## What's next
* [User Authentication](/docs/reference/access-authn-authz/authentication/)
* [Authenticating with Bootstrap Tokens](/docs/reference/access-authn-authz/bootstrap-tokens/)
* [kubelet Authentication](/docs/reference/access-authn-authz/kubelet-authn-authz/#kubelet-authentication)
* [Authenticating with Service Account Tokens](/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-tokens)
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified November 12, 2024 at 5:08 PM PST: [Add reference to authentication-mechanisms.md (cd0b9c3a0c)](https://github.com/kubernetes/website/commit/cd0b9c3a0c410527345d8041a10fa415d216a2ee)