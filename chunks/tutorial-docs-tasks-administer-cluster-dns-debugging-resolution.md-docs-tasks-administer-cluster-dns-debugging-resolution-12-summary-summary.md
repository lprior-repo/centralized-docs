---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#12-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 109
summary: ``` `pod/dnsutils created ` ``` …and verify its status: ``` `kubectl get pods dnsutils ` ``` ``` `NAME READY STATUS RESTARTS AGE dnsutils 1/1 Running 0 &lt;some-time&gt; ` ``` Once that Pod is...
---

```
`pod/dnsutils created
`
```
…and verify its status:
```
`kubectl get pods dnsutils
`
```
```
`NAME READY STATUS RESTARTS AGE
dnsutils 1/1 Running 0 &lt;some-time&gt;
`
```
Once that Pod is running, you can exec `nslookup` in that environment.
If you see something like the following, DNS is working correctly.
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```