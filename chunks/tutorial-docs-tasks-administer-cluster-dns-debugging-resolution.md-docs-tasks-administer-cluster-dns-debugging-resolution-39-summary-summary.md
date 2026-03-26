---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#39-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 119
summary: ### Are you in the right namespace for the service? DNS queries that don't specify a namespace are limited to the pod's namespace. If the namespace of the pod and service differ, the DNS query must...
---

### Are you in the right namespace for the service?
DNS queries that don't specify a namespace are limited to the pod's
namespace.
If the namespace of the pod and service differ, the DNS query must include
the namespace of the service.
This query is limited to the pod's namespace:
```
`kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;
`
```
This query specifies the namespace:
```
`kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;.&lt;namespace&gt;
`
```