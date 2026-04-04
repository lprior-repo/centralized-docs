---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#1-detailed
chunk_level: detailed
chunk_type: code
heading: Before you begin
token_count: 926
summary: # Validate IPv4/IPv6 dual-stack This document shares how to validate IPv4/IPv6 dual-stack enabled Kubernetes clusters. ## Before you begin * Provider support for dual-stack networking (Cloud provider...
---

# Validate IPv4/IPv6 dual-stack
This document shares how to validate IPv4/IPv6 dual-stack enabled Kubernetes clusters.
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
### Validate Pod addressing
Validate that a Pod has an IPv4 and IPv6 address assigned. Replace the Pod name with
a valid Pod in your cluster. In this example the Pod name is `pod01`:
```
`kubectl get pods pod01 -o go-template --template='{{range .status.podIPs}}{{printf "%s\\n" .ip}}{{end}}'
`
```
```
`10.244.1.4
2001:db8::4
`
```
You can also validate Pod IPs using the Downward API via the `status.podIPs` fieldPath.
The following snippet demonstrates how you can expose the Pod IPs via an environment variable
called `MY\_POD\_IPS` within a container.
```
` env:
- name: MY\_POD\_IPS
valueFrom:
fieldRef:
fieldPath: status.podIPs
`
```
The following command prints the value of the `MY\_POD\_IPS` environment variable from
within a container. The value is a comma separated list that corresponds to the
Pod's IPv4 and IPv6 addresses.
```
`kubectl exec -it pod01 -- set | grep MY\_POD\_IPS
`
```
```
`MY\_POD\_IPS=10.244.1.4,2001:db8::4
`
```
The Pod's IP addresses will also be written to `/etc/hosts` within a container.
The following command executes a cat on `/etc/hosts` on a dual stack Pod.
From the output you can verify both the IPv4 and IPv6 IP address for the Pod.
```
`kubectl exec -it pod01 -- cat /etc/hosts
`
```
```
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.244.1.4 pod01
2001:db8::4 pod01
`
```