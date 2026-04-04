---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 29
summary: ``` `./easyrsa --batch \"--req-cn=${MASTER\_IP}@`date +%s`\" build-ca nopass ` ```
---

```
`./easyrsa --batch "--req-cn=${MASTER\_IP}@`date +%s`" build-ca nopass
`
```