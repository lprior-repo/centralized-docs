---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#23-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: ``` `../cfssl gencert -initca ca-csr.json | ../cfssljson -bare ca ` ``` 6. Create a JSON config file for generating keys and certificates for the API server, for example, `server-csr.json`. Be sure...
---

```
`../cfssl gencert -initca ca-csr.json | ../cfssljson -bare ca
`
```
6. Create a JSON config file for generating keys and certificates for the API
server, for example, `server-csr.json`. Be sure to replace the values in angle brackets with
real values you want to use. The `&lt;MASTER\_CLUSTER\_IP&gt;` is the service cluster
IP for the API server as described in previous subsection.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.