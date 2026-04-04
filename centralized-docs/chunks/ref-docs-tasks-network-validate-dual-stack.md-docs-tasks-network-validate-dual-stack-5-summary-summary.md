---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#5-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 85
summary: ``` `10.244.1.0/24 2001:db8::/64 ` ``` There should be one IPv4 block and one IPv6 block allocated. Validate that the node has an IPv4 and IPv6 interface detected. Replace node name with a valid node...
---

```
`10.244.1.0/24
2001:db8::/64
`
```
There should be one IPv4 block and one IPv6 block allocated.
Validate that the node has an IPv4 and IPv6 interface detected.
Replace node name with a valid node from the cluster.
In this example the node name is `k8s-linuxpool1-34450317-0`: