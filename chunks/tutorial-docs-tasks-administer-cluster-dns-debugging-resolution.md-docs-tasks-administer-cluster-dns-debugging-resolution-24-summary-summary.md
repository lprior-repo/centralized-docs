---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#24-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 91
summary: ### Is DNS service up? Verify that the DNS service is up by using the `kubectl get service` command. ``` `kubectl get svc --namespace=kube-system ` ``` ``` `NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S)...
---

### Is DNS service up?
Verify that the DNS service is up by using the `kubectl get service` command.
```
`kubectl get svc --namespace=kube-system
`
```
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
...
kube-dns ClusterIP 10.0.0.10 &lt;none&gt; 53/UDP,53/TCP 1h
...
`
```