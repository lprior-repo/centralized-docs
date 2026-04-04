---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#14-summary
chunk_level: summary
chunk_type: prose
heading: Control plane to node
token_count: 81
summary: 's serving certificate. If that is not possible, use [SSH tunneling](#ssh-tunnels) between the API server and kubelet if required to avoid connecting over an untrusted or public network. Finally,...
---

's serving certificate.
If that is not possible, use [SSH tunneling](#ssh-tunnels) between the API server and kubelet if
required to avoid connecting over an
untrusted or public network.
Finally, [Kubelet authentication and/or authorization](/docs/reference/access-authn-authz/kubelet-authn-authz/)
should be enabled to secure the kubelet API.