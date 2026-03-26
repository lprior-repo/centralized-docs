---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#36-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 114
summary: ``` `2022-03-18T07:12:15.699431183Z [INFO] 10.96.144.227:52299 - 3686 \"A IN serverproxy.contoso.net.cluster.local. udp 52 false 512\" SERVFAIL qr,aa,rd 145 0.000091221s ` ``` First, get the current...
---

```
`2022-03-18T07:12:15.699431183Z [INFO] 10.96.144.227:52299 - 3686 "A IN serverproxy.contoso.net.cluster.local. udp 52 false 512" SERVFAIL qr,aa,rd 145 0.000091221s
`
```
First, get the current ClusterRole of `system:coredns`:
```
`kubectl describe clusterrole system:coredns -n kube-system
`
```
Expected output: