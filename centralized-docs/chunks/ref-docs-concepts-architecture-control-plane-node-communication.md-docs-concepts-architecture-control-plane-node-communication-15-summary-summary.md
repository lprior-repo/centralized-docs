---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#15-summary
chunk_level: summary
chunk_type: prose
heading: Control plane to node
token_count: 123
summary: ### API server to nodes, pods, and services The connections from the API server to a node, pod, or service default to plain HTTP connections and are therefore neither authenticated nor encrypted....
---

### API server to nodes, pods, and services
The connections from the API server to a node, pod, or service default to plain HTTP connections
and are therefore neither authenticated nor encrypted. They can be run over a secure HTTPS
connection by prefixing `https:` to the node, pod, or service name in the API URL, but they will
not validate the certificate provided by the HTTPS endpoint nor provide client credentials. So
while the connection will be encrypted, it will not provide any guarantees of integrity. These
connections **are not currently safe** to run over untrusted or public networks.