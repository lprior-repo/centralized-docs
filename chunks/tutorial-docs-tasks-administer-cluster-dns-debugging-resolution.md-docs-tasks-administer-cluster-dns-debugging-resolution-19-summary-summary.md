---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#19-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 72
summary: #### Note: The value for label `k8s-app` is `kube-dns` for both CoreDNS and kube-dns deployments. If you see that no CoreDNS Pod is running or that the Pod has failed/completed, the DNS add-on may...
---

#### Note:
The value for label `k8s-app` is `kube-dns` for both CoreDNS and kube-dns deployments.
If you see that no CoreDNS Pod is running or that the Pod has failed/completed,
the DNS add-on may not be deployed by default in your current environment and you
will have to deploy it manually.