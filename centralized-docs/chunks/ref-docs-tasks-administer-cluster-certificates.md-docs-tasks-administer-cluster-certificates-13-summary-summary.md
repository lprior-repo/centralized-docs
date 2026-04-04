---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#13-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 104
summary: authorityKeyIdentifier=keyid,issuer:always basicConstraints=CA:FALSE keyUsage=keyEncipherment,dataEncipherment extendedKeyUsage=serverAuth,clientAuth subjectAltName=@alt\_names ` ``` 5. Generate the...
---

authorityKeyIdentifier=keyid,issuer:always
basicConstraints=CA:FALSE
keyUsage=keyEncipherment,dataEncipherment
extendedKeyUsage=serverAuth,clientAuth
subjectAltName=@alt\_names
`
```
5. Generate the certificate signing request based on the config file:
```
`openssl req -new -key server.key -out server.csr -config csr.conf
`
```
6. Generate the server certificate using the ca.key, ca.crt and server.csr: