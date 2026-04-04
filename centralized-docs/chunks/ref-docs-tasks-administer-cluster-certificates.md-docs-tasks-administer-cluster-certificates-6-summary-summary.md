---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 81
summary: 4. Copy `pki/ca.crt`, `pki/issued/server.crt`, and `pki/private/server.key` to your directory. 5. Fill in and add the following parameters into the API server start parameters: ```...
---

4. Copy `pki/ca.crt`, `pki/issued/server.crt`, and `pki/private/server.key` to your directory.
5. Fill in and add the following parameters into the API server start parameters:
```
`--client-ca-file=/yourdirectory/ca.crt
--tls-cert-file=/yourdirectory/server.crt
--tls-private-key-file=/yourdirectory/server.key
`
```