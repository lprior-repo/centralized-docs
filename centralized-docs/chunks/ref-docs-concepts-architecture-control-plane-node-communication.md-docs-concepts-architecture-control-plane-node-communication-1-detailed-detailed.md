---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Node to Control Plane
token_count: 525
summary: # Communication between Nodes and the Control Plane This document catalogs the communication paths between the [API server](/docs/concepts/architecture/#kube-apiserver) and the Kubernetes...
---

# Communication between Nodes and the Control Plane
This document catalogs the communication paths between the [API server](/docs/concepts/architecture/#kube-apiserver)
and the Kubernetes [cluster](/docs/reference/glossary/?all=true#term-cluster).
The intent is to allow users to customize their installation to harden the network configuration
such that the cluster can be run on an untrusted network (or on fully public IPs on a cloud
provider).
## Node to Control Plane
Kubernetes has a "hub-and-spoke" API pattern. All API usage from nodes (or the pods they run)
terminates at the API server. None of the other control plane components are designed to expose
remote services. The API server is configured to listen for remote connections on a secure HTTPS
port (typically 443) with one or more forms of client
[authentication](/docs/reference/access-authn-authz/authentication/) enabled.
One or more forms of [authorization](/docs/reference/access-authn-authz/authorization/) should be
enabled, especially if [anonymous requests](/docs/reference/access-authn-authz/authentication/#anonymous-requests)
or [service account tokens](/docs/reference/access-authn-authz/authentication/#service-account-tokens)
are allowed.
Nodes should be provisioned with the public root [certificate](/docs/tasks/tls/managing-tls-in-a-cluster/) for the cluster such that they can
connect securely to the API server along with valid client credentials. A good approach is that the
client credentials provided to the kubelet are in the form of a client certificate. See
[kubelet TLS bootstrapping](/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/)
for automated provisioning of kubelet client certificates.
[Pods](/docs/concepts/workloads/pods/) that wish to connect to the API server can do so securely by leveraging a service account so
that Kubernetes will automatically inject the public root certificate and a valid bearer token
into the pod when it is instantiated.
The `kubernetes` service (in `default` namespace) is configured with a virtual IP address that is
redirected (via `[kube-proxy](/docs/reference/command-line-tools-reference/kube-proxy/)`) to the HTTPS endpoint on the API server.
The control plane components also communicate with the API server over the secure port.
As a result, the default operating mode for connections from the nodes and pod running on the
nodes to the control plane is secured by default and can run over untrusted and/or public
networks.