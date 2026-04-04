---
doc_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication
chunk_id: ref/docs-concepts-architecture-control-plane-node-communication.md/docs-concepts-architecture-control-plane-node-communication#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 93
summary: # Communication between Nodes and the Control Plane This document catalogs the communication paths between the [API server](/docs/concepts/architecture/#kube-apiserver) and the Kubernetes...
---

# Communication between Nodes and the Control Plane
This document catalogs the communication paths between the [API server](/docs/concepts/architecture/#kube-apiserver)
and the Kubernetes [cluster](/docs/reference/glossary/?all=true#term-cluster).
The intent is to allow users to customize their installation to harden the network configuration
such that the cluster can be run on an untrusted network (or on fully public IPs on a cloud
provider).