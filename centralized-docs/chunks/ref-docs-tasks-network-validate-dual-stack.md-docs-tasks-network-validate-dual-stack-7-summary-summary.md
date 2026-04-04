---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 104
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