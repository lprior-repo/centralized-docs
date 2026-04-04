---
doc_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms
chunk_id: ref/docs-concepts-security-hardening-guide-authentication-mechanisms.md/docs-concepts-security-hardening-guide-authentication-mechanisms#3-standard
chunk_level: standard
chunk_type: prose
heading: X.509 client certificate authentication
token_count: 447
summary: * Client certificates cannot be individually revoked. Once compromised, a certificate can be used by an attacker until it expires. To mitigate this risk, it is recommended to configure short...
---

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