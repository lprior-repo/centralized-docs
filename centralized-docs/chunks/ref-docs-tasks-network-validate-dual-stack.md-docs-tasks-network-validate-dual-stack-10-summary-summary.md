---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 103
summary: ``` `MY\_POD\_IPS=10.244.1.4,2001:db8::4 ` ``` The Pod's IP addresses will also be written to `/etc/hosts` within a container. The following command executes a cat on `/etc/hosts` on a dual stack...
---

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