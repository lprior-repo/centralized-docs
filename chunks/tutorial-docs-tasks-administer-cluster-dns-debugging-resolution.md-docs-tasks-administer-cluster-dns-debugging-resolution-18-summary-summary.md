---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#18-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 115
summary: ### Check if the DNS pod is running Use the `kubectl get pods` command to verify that the DNS pod is running. ``` `kubectl get pods --namespace=kube-system -l k8s-app=kube-dns ` ``` ``` `NAME READY...
---

### Check if the DNS pod is running
Use the `kubectl get pods` command to verify that the DNS pod is running.
```
`kubectl get pods --namespace=kube-system -l k8s-app=kube-dns
`
```
```
`NAME READY STATUS RESTARTS AGE
...
coredns-7b96bf9f76-5hsxb 1/1 Running 0 1h
coredns-7b96bf9f76-mvmmt 1/1 Running 0 1h
...
`
```