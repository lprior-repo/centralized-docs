---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#13-summary
chunk_level: summary
chunk_type: prose
heading: Control plane to node
token_count: 122
summary: * Providing the kubelet's port-forwarding functionality. These connections terminate at the kubelet's HTTPS endpoint. By default, the API server does not verify the kubelet's serving certificate,...
---

* Providing the kubelet's port-forwarding functionality.
These connections terminate at the kubelet's HTTPS endpoint. By default, the API server does not
verify the kubelet's serving certificate, which makes the connection subject to man-in-the-middle
attacks and **unsafe** to run over untrusted and/or public networks.
To verify this connection, use the `--kubelet-certificate-authority` flag to provide the API
server with a root certificate bundle to use to verify the kubelet's serving certificate.
If that is not possible, use [SSH tunneling](#ssh-tunnels)