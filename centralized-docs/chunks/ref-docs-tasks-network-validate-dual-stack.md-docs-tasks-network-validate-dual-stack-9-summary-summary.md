---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#9-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 96
summary: The following command prints the value of the `MY\_POD\_IPS` environment variable from within a container. The value is a comma separated list that corresponds to the Pod's IPv4 and IPv6 addresses....
---

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