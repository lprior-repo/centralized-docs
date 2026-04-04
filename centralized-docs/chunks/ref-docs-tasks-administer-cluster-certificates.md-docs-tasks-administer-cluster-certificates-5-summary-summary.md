---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 93
summary: ``` `./easyrsa --subject-alt-name=\"IP:${MASTER\_IP},\"\\ \"IP:${MASTER\_CLUSTER\_IP},\"\\ \"DNS:kubernetes,\"\\ \"DNS:kubernetes.default,\"\\ \"DNS:kubernetes.default.svc,\"\\...
---

```
`./easyrsa --subject-alt-name="IP:${MASTER\_IP},"\\
"IP:${MASTER\_CLUSTER\_IP},"\\
"DNS:kubernetes,"\\
"DNS:kubernetes.default,"\\
"DNS:kubernetes.default.svc,"\\
"DNS:kubernetes.default.svc.cluster,"\\
"DNS:kubernetes.default.svc.cluster.local" \\
--days=10000 \\
build-server-full server nopass
`
```