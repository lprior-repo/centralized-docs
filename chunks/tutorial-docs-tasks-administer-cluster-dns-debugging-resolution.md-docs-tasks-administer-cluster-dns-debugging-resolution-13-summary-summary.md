---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#13-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 83
summary: ``` `kubectl exec -i -t dnsutils -- nslookup kubernetes.default ` ``` ``` `Server: 10.0.0.10 Address 1: 10.0.0.10 Name: kubernetes.default Address 1: 10.0.0.1 ` ``` If the `nslookup` command fails,...
---

```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10
Name: kubernetes.default
Address 1: 10.0.0.1
`
```
If the `nslookup` command fails, check the following: