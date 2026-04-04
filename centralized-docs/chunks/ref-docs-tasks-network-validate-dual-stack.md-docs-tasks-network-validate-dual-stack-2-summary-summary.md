---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#2-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 106
summary: ## Before you begin * Provider support for dual-stack networking (Cloud provider or otherwise must be able to provide Kubernetes nodes with routable IPv4/IPv6 network interfaces) * A [network...
---

## Before you begin
* Provider support for dual-stack networking (Cloud provider or otherwise must be able to
provide Kubernetes nodes with routable IPv4/IPv6 network interfaces)
* A [network plugin](/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/)
that supports dual-stack networking.
* [Dual-stack enabled](/docs/concepts/services-networking/dual-stack/) clusterYour Kubernetes server must be at or later than version v1.23.
To check the version, enter `kubectl version`.