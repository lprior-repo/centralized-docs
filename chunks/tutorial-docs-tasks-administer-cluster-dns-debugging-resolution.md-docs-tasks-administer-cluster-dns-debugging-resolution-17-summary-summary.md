---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#17-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 73
summary: or ``` `kubectl exec -i -t dnsutils -- nslookup kubernetes.default ` ``` ``` `Server: 10.0.0.10 Address 1: 10.0.0.10 kube-dns.kube-system.svc.cluster.local nslookup: can't resolve...
---

or
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10 kube-dns.kube-system.svc.cluster.local
nslookup: can't resolve 'kubernetes.default'
`
```