---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#4-standard
chunk_level: standard
chunk_type: prose
heading: Controlling access to the Kubernetes API
token_count: 298
summary: ## Controlling access to the Kubernetes API As Kubernetes is entirely API-driven, controlling and limiting who can access the cluster and what actions they are allowed to perform is the first line of...
---

## Controlling access to the Kubernetes API
As Kubernetes is entirely API-driven, controlling and limiting who can access the cluster and what actions
they are allowed to perform is the first line of defense.
### Use Transport Layer Security (TLS) for all API traffic
Kubernetes expects that all API communication in the cluster is encrypted by default with TLS, and the
majority of installation methods will allow the necessary certificates to be created and distributed to
the cluster components. Note that some components and installation methods may enable local ports over
HTTP and administrators should familiarize themselves with the settings of each component to identify
potentially unsecured traffic.
### API Authentication
Choose an authentication mechanism for the API servers to use that matches the common access patterns
when you install a cluster. For instance, small, single-user clusters may wish to use a simple certificate
or static Bearer token approach. Larger clusters may wish to integrate an existing OIDC or LDAP server that
allow users to be subdivided into groups.
All API clients must be authenticated, even those that are part of the infrastructure like nodes,
proxies, the scheduler, and volume plugins. These clients are typically [service accounts](/docs/reference/access-authn-authz/service-accounts-admin/) or use x509 client certificates, and they are created automatically at cluster startup or are setup as part of the cluster installation.
Consult the [authentication reference document](/docs/reference/access-authn-authz/authentication/) for more information.