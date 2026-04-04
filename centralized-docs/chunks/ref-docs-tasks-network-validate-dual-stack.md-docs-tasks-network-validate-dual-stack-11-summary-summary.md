---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 110
summary: ``` `kubectl exec -it pod01 -- cat /etc/hosts ` ``` ``` `# Kubernetes-managed hosts file. 127.0.0.1 localhost ::1 localhost ip6-localhost ip6-loopback fe00::0 ip6-localnet fe00::0 ip6-mcastprefix...
---

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