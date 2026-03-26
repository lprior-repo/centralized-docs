---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#28-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 104
summary: ### Are DNS queries being received/processed? You can verify if queries are being received by CoreDNS by adding the `log` plugin to the CoreDNS configuration (aka Corefile). The CoreDNS Corefile is...
---

### Are DNS queries being received/processed?
You can verify if queries are being received by CoreDNS by adding the `log` plugin to the CoreDNS configuration (aka Corefile).
The CoreDNS Corefile is held in a [ConfigMap](/docs/concepts/configuration/configmap/) named `coredns`. To edit it, use the command:
```
`kubectl -n kube-system edit configmap coredns
`
```
Then add `log` in the Corefile section per the example below: