---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Controlling access to the Kubelet
token_count: 1001
summary: # Securing a Cluster This document covers topics related to protecting a cluster from accidental or malicious access and provides recommendations on overall security. ## Before you begin * You need...
---

# Securing a Cluster
This document covers topics related to protecting a cluster from accidental or malicious access
and provides recommendations on overall security.
## Before you begin
* You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
To check the version, enter `kubectl version`.
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
### API Authorization
Once authenticated, every API call is also expected to pass an authorization check. Kubernetes ships
an integrated [Role-Based Access Control (RBAC)](/docs/reference/access-authn-authz/rbac/) component that matches an incoming user or group to a
set of permissions bundled into roles. These permissions combine verbs (get, create, delete) with
resources (pods, services, nodes) and can be namespace-scoped or cluster-scoped. A set of out-of-the-box
roles are provided that offer reasonable default separation of responsibility depending on what
actions a client might want to perform. It is recommended that you use the
[Node](/docs/reference/access-authn-authz/node/) and
[RBAC](/docs/reference/access-authn-authz/rbac/) authorizers together, in combination with the
[NodeRestriction](/docs/reference/access-authn-authz/admission-controllers/#noderestriction) admission plugin.
As with authentication, simple and broad roles may be appropriate for smaller clusters, but as
more users interact with the cluster, it may become necessary to separate teams into separate
[namespaces](/docs/concepts/overview/working-with-objects/namespaces) with more limited roles.
With authorization, it is important to understand how updates on one object may cause actions in
other places. For instance, a user may not be able to create pods directly, but allowing them to
create a deployment, which creates pods on their behalf, will let them create those pods
indirectly. Likewise, deleting a node from the API will result in the pods scheduled to that node
being terminated and recreated on other nodes. The out-of-the box roles represent a balance
between flexibility and common use cases, but more limited roles should be carefully reviewed
to prevent accidental escalation. You can make roles specific to your use case if the out-of-box ones don't meet your needs.
Consult the [authorization reference section](/docs/reference/access-authn-authz/authorization/) for more information.
## Controlling access to the Kubelet
Kubelets expose HTTPS endpoints which grant powerful control over the node and containers.
By default Kubelets allow unauthenticated access to this API.
Production clusters should enable Kubelet authentication and authorization.
Consult the [Kubelet authentication/authorization reference](/docs/reference/access-authn-authz/kubelet-authn-authz/)
for more information.