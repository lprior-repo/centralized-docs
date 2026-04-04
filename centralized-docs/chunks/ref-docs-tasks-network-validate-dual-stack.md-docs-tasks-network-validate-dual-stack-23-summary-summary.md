---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#23-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 55
summary: Validate that the Service gets cluster IPs from the IPv4 and IPv6 address blocks using `kubectl describe`. You may then validate access to the service via the IPs and ports. ``` `kubectl describe svc...
---

Validate that the Service gets cluster IPs from the IPv4 and IPv6 address blocks using
`kubectl describe`. You may then validate access to the service via the IPs and ports.
```
`kubectl describe svc -l app.kubernetes.io/name=MyApp
`
```