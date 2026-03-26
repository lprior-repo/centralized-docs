---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#16-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 84
summary: ``` `kubectl exec -i -t dnsutils -- nslookup kubernetes.default ` ``` ``` `Server: 10.0.0.10 Address 1: 10.0.0.10 nslookup: can't resolve 'kubernetes.default' ` ``` or ``` `kubectl exec -i -t...
---

```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10
nslookup: can't resolve 'kubernetes.default'
`
```
or
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```