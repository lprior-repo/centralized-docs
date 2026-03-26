---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#20-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 63
summary: ### Check for errors in the DNS pod Use the `kubectl logs` command to see logs for the DNS containers. For CoreDNS: ``` `kubectl logs --namespace=kube-system -l k8s-app=kube-dns ` ``` Here is an...
---

### Check for errors in the DNS pod
Use the `kubectl logs` command to see logs for the DNS containers.
For CoreDNS:
```
`kubectl logs --namespace=kube-system -l k8s-app=kube-dns
`
```
Here is an example of a healthy CoreDNS log: