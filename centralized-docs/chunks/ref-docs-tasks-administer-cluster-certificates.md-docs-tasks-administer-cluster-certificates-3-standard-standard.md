---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 177
summary: 5. Generate the certificate signing request based on the config file: ``` `openssl req -new -key server.key -out server.csr -config csr.conf ` ``` 6. Generate the server certificate using the ca.key,...
---

5. Generate the certificate signing request based on the config file:
```
`openssl req -new -key server.key -out server.csr -config csr.conf
`
```
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
Finally, add the same parameters into the API server start parameters.