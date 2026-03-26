---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#26-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 97
summary: ### Are DNS endpoints exposed? You can verify that DNS endpoints are exposed by using the `kubectl get endpointslice` command. ``` `kubectl get endpointslice -l kubernetes.io/service-name=kube-dns...
---

### Are DNS endpoints exposed?
You can verify that DNS endpoints are exposed by using the `kubectl get endpointslice`
command.
```
`kubectl get endpointslice -l kubernetes.io/service-name=kube-dns --namespace=kube-system
`
```
```
`NAME ADDRESSTYPE PORTS ENDPOINTS AGE
kube-dns-zxoja IPv4 53 10.180.3.17,10.180.3.17 1h
`
```