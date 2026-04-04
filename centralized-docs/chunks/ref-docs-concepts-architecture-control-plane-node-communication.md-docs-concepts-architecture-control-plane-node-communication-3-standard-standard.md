---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#3-standard
chunk_level: standard
chunk_type: prose
heading: Control plane to node
token_count: 430
summary: ## Control plane to node There are two primary communication paths from the control plane (the API server) to the nodes. The first is from the API server to the...
---

## Control plane to node
There are two primary communication paths from the control plane (the API server) to the nodes.
The first is from the API server to the [kubelet](/docs/reference/command-line-tools-reference/kubelet) process which runs on each node in the cluster.
The second is from the API server to any node, pod, or service through the API server's *proxy*
functionality.
### API server to kubelet
The connections from the API server to the kubelet are used for:
* Fetching logs for pods.
* Attaching (usually through `kubectl`) to running pods.
* Providing the kubelet's port-forwarding functionality.
These connections terminate at the kubelet's HTTPS endpoint. By default, the API server does not
verify the kubelet's serving certificate, which makes the connection subject to man-in-the-middle
attacks and **unsafe** to run over untrusted and/or public networks.
To verify this connection, use the `--kubelet-certificate-authority` flag to provide the API
server with a root certificate bundle to use to verify the kubelet's serving certificate.
If that is not possible, use [SSH tunneling](#ssh-tunnels) between the API server and kubelet if
required to avoid connecting over an
untrusted or public network.
Finally, [Kubelet authentication and/or authorization](/docs/reference/access-authn-authz/kubelet-authn-authz/)
should be enabled to secure the kubelet API.
### API server to nodes, pods, and services
The connections from the API server to a node, pod, or service default to plain HTTP connections
and are therefore neither authenticated nor encrypted. They can be run over a secure HTTPS
connection by prefixing `https:` to the node, pod, or service name in the API URL, but they will
not validate the certificate provided by the HTTPS endpoint nor provide client credentials. So
while the connection will be encrypted, it will not provide any guarantees of integrity. These
connections **are not currently safe** to run over untrusted or public networks.