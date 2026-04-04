---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#14-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: 6. Generate the server certificate using the ca.key, ca.crt and server.csr: ``` `openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key \\ -CAcreateserial -out server.crt -days 10000 \\...
---

6. Generate the server certificate using the ca.key, ca.crt and server.csr:
```
`openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key \\
-CAcreateserial -out server.crt -days 10000 \\
-extensions v3\_ext -extfile csr.conf -sha256
`
```
7. View the certificate signing request:
```
`openssl req -noout -text -in ./server.csr
`
```
8. View the certificate:
```
`openssl x509 -noout -text -in ./server.crt
`
```