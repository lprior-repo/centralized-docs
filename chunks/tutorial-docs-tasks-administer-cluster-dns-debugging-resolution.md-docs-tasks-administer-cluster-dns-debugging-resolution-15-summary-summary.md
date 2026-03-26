---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#15-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: Verify that the search path and name server are set up like the following (note that search path may vary for different cloud providers): ``` `search default.svc.cluster.local svc.cluster.local...
---

Verify that the search path and name server are set up like the following
(note that search path may vary for different cloud providers):
```
`search default.svc.cluster.local svc.cluster.local cluster.local google.internal c.gce\_project\_id.internal
nameserver 10.0.0.10
options ndots:5
`
```
Errors such as the following indicate a problem with the CoreDNS (or kube-dns)
add-on or with associated Services:
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```