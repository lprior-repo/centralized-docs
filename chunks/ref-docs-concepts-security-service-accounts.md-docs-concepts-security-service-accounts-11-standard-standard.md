---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#11-standard
chunk_level: standard
chunk_type: prose
heading: Alternatives
token_count: 410
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