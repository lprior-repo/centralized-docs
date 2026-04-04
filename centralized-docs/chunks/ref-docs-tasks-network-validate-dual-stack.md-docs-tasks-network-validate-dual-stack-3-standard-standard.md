---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#3-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 453
summary: ### Validate Pod addressing Validate that a Pod has an IPv4 and IPv6 address assigned. Replace the Pod name with a valid Pod in your cluster. In this example the Pod name is `pod01`: ``` `kubectl get...
---

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