---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#37-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 125
summary: ``` `kubectl describe clusterrole system:coredns -n kube-system ` ``` Expected output: ``` `PolicyRule: Resources Non-Resource URLs Resource Names Verbs --------- ----------------- --------------...
---

```
`kubectl describe clusterrole system:coredns -n kube-system
`
```
Expected output:
```
`PolicyRule:
Resources Non-Resource URLs Resource Names Verbs
--------- ----------------- -------------- -----
endpoints [] [] [list watch]
namespaces [] [] [list watch]
pods [] [] [list watch]
services [] [] [list watch]
endpointslices.discovery.k8s.io [] [] [list watch]
`
```
If any permissions are missing, edit the ClusterRole to add them:
```
`kubectl edit clusterrole system:coredns -n kube-system
`
```