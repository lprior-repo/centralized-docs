---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#40-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 67
summary: ``` `kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;.&lt;namespace&gt; ` ``` To learn more about name resolution, see [DNS for Services and...
---

```
`kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;.&lt;namespace&gt;
`
```
To learn more about name resolution, see
[DNS for Services and Pods](/docs/concepts/services-networking/dns-pod-service/#what-things-get-dns-names).