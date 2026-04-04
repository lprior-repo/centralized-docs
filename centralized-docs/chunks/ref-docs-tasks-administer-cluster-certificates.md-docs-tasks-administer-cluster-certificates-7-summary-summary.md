---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 117
summary: ### openssl **openssl** can manually generate certificates for your cluster. 1. Generate a ca.key with 2048bit: ``` `openssl genrsa -out ca.key 2048 ` ``` 2. According to the ca.key generate a ca.crt...
---

### openssl
**openssl** can manually generate certificates for your cluster.
1. Generate a ca.key with 2048bit:
```
`openssl genrsa -out ca.key 2048
`
```
2. According to the ca.key generate a ca.crt (use `-days` to set the certificate effective time):
```
`openssl req -x509 -new -noenc -key ca.key -subj "/CN=${MASTER\_IP}" -days 10000 -out ca.crt
`
```
3. Generate a server.key with 2048bit: