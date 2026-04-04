---
doc_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access
chunk_id: ref/docs-concepts-security-controlling-access.md/docs-concepts-security-controlling-access#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Authentication
token_count: 574
summary: # Controlling Access to the Kubernetes API This page provides an overview of controlling access to the Kubernetes API. Users access the [Kubernetes API](/docs/concepts/overview/kubernetes-api/) using...
---

# Controlling Access to the Kubernetes API
This page provides an overview of controlling access to the Kubernetes API.
Users access the [Kubernetes API](/docs/concepts/overview/kubernetes-api/) using `kubectl`,
client libraries, or by making REST requests. Both human users and
[Kubernetes service accounts](/docs/tasks/configure-pod-container/configure-service-account/) can be
authorized for API access.
When a request reaches the API, it goes through several stages, illustrated in the
following diagram:
![Diagram of request handling steps for Kubernetes API request](/images/docs/admin/access-control-overview.svg)
## Transport security
By default, the Kubernetes API server listens on port 6443 on the first non-localhost
network interface, protected by TLS. In a typical production Kubernetes cluster, the
API serves on port 443. The port can be changed with the `--secure-port`, and the
listening IP address with the `--bind-address` flag.
The API server presents a certificate. This certificate may be signed using
a private certificate authority (CA), or based on a public key infrastructure linked
to a generally recognized CA. The certificate and corresponding private key can be set
by using the `--tls-cert-file` and `--tls-private-key-file` flags.
If your cluster uses a private certificate authority, you need a copy of that CA
certificate configured into your `\~/.kube/config` on the client, so that you can
trust the connection and be confident it was not intercepted.
Your client can present a TLS client certificate at this stage.
## Authentication
Once TLS is established, the HTTP request moves to the Authentication step.
This is shown as step **1** in the diagram.
The cluster creation script or cluster admin configures the API server to run
one or more Authenticator modules.
Authenticators are described in more detail in
[Authentication](/docs/reference/access-authn-authz/authentication/).
The input to the authentication step is the entire HTTP request; however, it typically
examines the headers and/or client certificate.
Authentication modules include client certificates, password, and plain tokens,
bootstrap tokens, and JSON Web Tokens (used for service accounts).
Multiple authentication modules can be specified, in which case each one is tried in sequence,
until one of them succeeds.
If the request cannot be authenticated, it is rejected with HTTP status code 401.
Otherwise, the user is authenticated as a specific `username`, and the user name
is available to subsequent steps to use in their decisions. Some authenticators
also provide the group memberships of the user, while other authenticators
do not.
While Kubernetes uses usernames for access control decisions and in request logging,
it does not have a `User` object nor does it store usernames or other information about
users in its API.