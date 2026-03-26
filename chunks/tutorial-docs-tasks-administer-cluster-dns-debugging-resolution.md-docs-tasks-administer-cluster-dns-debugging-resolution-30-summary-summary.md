---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#30-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 70
summary: After saving the changes, it may take up to minute or two for Kubernetes to propagate these changes to the CoreDNS pods. Next, make some queries and view the logs per the sections above in this...
---

After saving the changes, it may take up to minute or two for Kubernetes to propagate these changes to the CoreDNS pods.
Next, make some queries and view the logs per the sections above in this document. If CoreDNS pods are receiving the queries, you should see them in the logs.
Here is an example of a query in the log: