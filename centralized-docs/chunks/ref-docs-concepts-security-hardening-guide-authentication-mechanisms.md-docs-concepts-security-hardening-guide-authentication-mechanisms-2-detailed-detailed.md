---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#2-detailed
chunk_level: detailed
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 814
summary: ## X.509 client certificate authentication Kubernetes leverages [X.509 client certificate](/docs/reference/access-authn-authz/authentication/#x509-client-certificates) authentication for system...
---

## X.509 client certificate authentication
Kubernetes leverages [X.509 client certificate](/docs/reference/access-authn-authz/authentication/#x509-client-certificates)
authentication for system components, such as when the kubelet authenticates to the API Server.
While this mechanism can also be used for user authentication, it might not be suitable for
production use due to several restrictions:
* Client certificates cannot be individually revoked. Once compromised, a certificate can be used
by an attacker until it expires. To mitigate this risk, it is recommended to configure short
lifetimes for user authentication credentials created using client certificates.
* If a certificate needs to be invalidated, the certificate authority must be re-keyed, which
can introduce availability risks to the cluster.
* There is no permanent record of client certificates created in the cluster. Therefore, all
issued certificates must be recorded if you need to keep track of them.
* Private keys used for client certificate authentication cannot be password-protected. Anyone
who can read the file containing the key will be able to make use of it.
* Using client certificate authentication requires a direct connection from the client to the
API server without any intervening TLS termination points, which can complicate network architectures.
* Group data is embedded in the `O` value of the client certificate, which means the user's group
memberships cannot be changed for the lifetime of the certificate.## Static token file
Although Kubernetes allows you to load credentials from a
[static token file](/docs/reference/access-authn-authz/authentication/#static-token-file) located
on the control plane node disks, this approach is not recommended for production servers due to
several reasons:
* Credentials are stored in clear text on control plane node disks, which can be a security risk.
* Changing any credential requires a restart of the API server process to take effect, which can
impact availability.
* There is no mechanism available to allow users to rotate their credentials. To rotate a
credential, a cluster administrator must modify the token on disk and distribute it to the users.
* There is no lockout mechanism available to prevent brute-force attacks.## Bootstrap tokens
[Bootstrap tokens](/docs/reference/access-authn-authz/bootstrap-tokens/) are used for joining
nodes to clusters and are not recommended for user authentication due to several reasons:
* They have hard-coded group memberships that are not suitable for general use, making them
unsuitable for authentication purposes.
* Manually generating bootstrap tokens can lead to weak tokens that can be guessed by an attacker,
which can be a security risk.
* There is no lockout mechanism available to prevent brute-force attacks, making it easier for
attackers to guess or crack the token.## ServiceAccount secret tokens
[Service account secrets](/docs/reference/access-authn-authz/service-accounts-admin/#manual-secret-management-for-serviceaccounts)
are available as an option to allow workloads running in the cluster to authenticate to the
API server. In Kubernetes &lt; 1.23, these were the default option, however, they are being replaced
with TokenRequest API tokens. While these secrets could be used for user authentication, they are
generally unsuitable for a number of reasons:
* They cannot be set with an expiry and will remain valid until the associated service account is deleted.
* The authentication tokens are visible to any cluster user who can read secrets in the namespace
that they are defined in.
* Service accounts cannot be added to arbitrary groups complicating RBAC management where they are used.## TokenRequest API tokens
The TokenRequest API is a useful tool for generating short-lived credentials for service
authentication to the API server or third-party systems. However, it is not generally recommended
for user authentication as there is no revocation method available, and distributing credentials
to users in a secure manner can be challenging.
When using TokenRequest tokens for service authentication, it is recommended to implement a short
lifespan to reduce the impact of compromised tokens.