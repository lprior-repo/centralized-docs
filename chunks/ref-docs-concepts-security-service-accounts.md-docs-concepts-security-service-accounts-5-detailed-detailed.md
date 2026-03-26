---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 788
summary: ## Alternatives * Issue your own tokens using another mechanism, and then use [Webhook Token Authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication) to...
---

## Alternatives
* Issue your own tokens using another mechanism, and then use
[Webhook Token Authentication](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication)
to validate bearer tokens using your own validation service.
* Provide your own identities to Pods.
* [Use the SPIFFE CSI driver plugin to provide SPIFFE SVIDs as X.509 certificate pairs to Pods](https://cert-manager.io/docs/projects/csi-driver-spiffe/).
🛇 This item links to a third party project or product that is not part of Kubernetes itself. [More information](#third-party-content-disclaimer)
* [Use a service mesh such as Istio to provide certificates to Pods](https://istio.io/latest/docs/tasks/security/cert-management/plugin-ca-cert/).
* Authenticate from outside the cluster to the API server without using service account tokens:
* [Configure the API server to accept OpenID Connect (OIDC) tokens from your identity provider](/docs/reference/access-authn-authz/authentication/#openid-connect-tokens).
* Use service accounts or user accounts created using an external Identity
and Access Management (IAM) service, such as from a cloud provider, to
authenticate to your cluster.
* [Use the CertificateSigningRequest API with client certificates](/docs/tasks/tls/managing-tls-in-a-cluster/).
* [Configure the kubelet to retrieve credentials from an image registry](/docs/tasks/administer-cluster/kubelet-credential-provider/).
* Use a Device Plugin to access a virtual Trusted Platform Module (TPM), which
then allows authentication using a private key.## What's next
* Learn how to [manage your ServiceAccounts as a cluster administrator](/docs/reference/access-authn-authz/service-accounts-admin/).
* Learn how to [assign a ServiceAccount to a Pod](/docs/tasks/configure-pod-container/configure-service-account/).
* Read the [ServiceAccount API reference](/docs/reference/kubernetes-api/authentication-resources/service-account-v1/).
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
Last modified November 19, 2024 at 10:53 PM PST: [Address comments (3b8c927a3b)](https://github.com/kubernetes/website/commit/3b8c927a3b369fbfde4d87321be94e0908548608)
Items on this page refer to third party products or projects that provide functionality required by Kubernetes. The Kubernetes project authors aren't responsible for those third-party products or projects. See the [CNCF website guidelines](https://github.com/cncf/foundation/blob/main/policies-guidance/website-guidelines.md) for more details.
You should read the [content guide](/docs/contribute/style/content-guide/#third-party-content) before proposing a change that adds an extra third-party link.
## Related Pages

- [Secrets](docs-concepts-configuration-secret.md)
- [Metrics for Kubernetes Object States](docs-concepts-cluster-administration-kube-state-metrics.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Owners and Dependents](docs-concepts-overview-working-with-objects-owners-dependents.md)
- [Other Tools](docs-reference-tools.md)