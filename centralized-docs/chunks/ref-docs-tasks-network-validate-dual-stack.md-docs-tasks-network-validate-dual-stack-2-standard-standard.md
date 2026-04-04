---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 445
summary: ## Before you begin * Provider support for dual-stack networking (Cloud provider or otherwise must be able to provide Kubernetes nodes with routable IPv4/IPv6 network interfaces) * A [network...
---

## Before you begin
* Provider support for dual-stack networking (Cloud provider or otherwise must be able to
provide Kubernetes nodes with routable IPv4/IPv6 network interfaces)
* A [network plugin](/docs/concepts/extend-kubernetes/compute-storage-net/network-plugins/)
that supports dual-stack networking.
* [Dual-stack enabled](/docs/concepts/services-networking/dual-stack/) clusterYour Kubernetes server must be at or later than version v1.23.
To check the version, enter `kubectl version`.
#### Note:
While you can validate with an earlier version, the feature is only GA and officially supported since v1.23.
### Validate node addressing
Each dual-stack Node should have a single IPv4 block and a single IPv6 block allocated.
Validate that IPv4/IPv6 Pod address ranges are configured by running the following command.
Replace the sample node name with a valid dual-stack Node from your cluster. In this example,
the Node's name is `k8s-linuxpool1-34450317-0`:
```
`kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .spec.podCIDRs}}{{printf "%s\\n" .}}{{end}}'
`
```
```
`10.244.1.0/24
2001:db8::/64
`
```
There should be one IPv4 block and one IPv6 block allocated.
Validate that the node has an IPv4 and IPv6 interface detected.
Replace node name with a valid node from the cluster.
In this example the node name is `k8s-linuxpool1-34450317-0`:
```
`kubectl get nodes k8s-linuxpool1-34450317-0 -o go-template --template='{{range .status.addresses}}{{printf "%s: %s\\n" .type .address}}{{end}}'
`
```
```
`Hostname: k8s-linuxpool1-34450317-0
InternalIP: 10.0.0.5
InternalIP: 2001:db8:10::5
`
```