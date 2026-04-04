---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 926
summary: ## OpenID Connect token authentication Kubernetes supports integrating external authentication services with the Kubernetes API using [OpenID Connect...
---

## OpenID Connect token authentication
Kubernetes supports integrating external authentication services with the Kubernetes API using
[OpenID Connect (OIDC)](/docs/reference/access-authn-authz/authentication/#openid-connect-tokens).
There is a wide variety of software that can be used to integrate Kubernetes with an identity
provider. However, when using OIDC authentication in Kubernetes, it is important to consider the
following hardening measures:
* The software installed in the cluster to support OIDC authentication should be isolated from
general workloads as it will run with high privileges.
* Some Kubernetes managed services are limited in the OIDC providers that can be used.
* As with TokenRequest tokens, OIDC tokens should have a short lifespan to reduce the impact of
compromised tokens.## Webhook token authentication
[Webhook token authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication)
is another option for integrating external authentication providers into Kubernetes. This mechanism
allows for an authentication service, either running inside the cluster or externally, to be
contacted for an authentication decision over a webhook. It is important to note that the suitability
of this mechanism will likely depend on the software used for the authentication service, and there
are some Kubernetes-specific considerations to take into account.
To configure Webhook authentication, access to control plane server filesystems is required. This
means that it will not be possible with Managed Kubernetes unless the provider specifically makes it
available. Additionally, any software installed in the cluster to support this access should be
isolated from general workloads, as it will run with high privileges.
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
## Related Pages

- [Controlling Access to the Kubernetes API](docs-concepts-security-controlling-access.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [kubelet authn authz](docs-reference-access-authn-authz-kubelet-authn-authz.md)
- [Implementation details](docs-reference-setup-tools-kubeadm-implementation-details.md)