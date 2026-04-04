---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 106
summary: ``` `10.244.1.4 2001:db8::4 ` ``` You can also validate Pod IPs using the Downward API via the `status.podIPs` fieldPath. The following snippet demonstrates how you can expose the Pod IPs via an...
---

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