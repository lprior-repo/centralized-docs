---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#22-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 44
summary: 5. Generate CA key (`ca-key.pem`) and certificate (`ca.pem`): ``` `../cfssl gencert -initca ca-csr.json | ../cfssljson -bare ca ` ```
---

5. Generate CA key (`ca-key.pem`) and certificate (`ca.pem`):
```
`../cfssl gencert -initca ca-csr.json | ../cfssljson -bare ca
`
```