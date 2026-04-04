---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 124
summary: ``` `openssl genrsa -out server.key 2048 ` ``` 4. Create a config file for generating a Certificate Signing Request (CSR). Be sure to substitute the values marked with angle brackets (e.g....
---

```
`openssl genrsa -out server.key 2048
`
```
4. Create a config file for generating a Certificate Signing Request (CSR).
Be sure to substitute the values marked with angle brackets (e.g. `&lt;MASTER\_IP&gt;`)
with real values before saving this to a file (e.g. `csr.conf`).
Note that the value for `MASTER\_CLUSTER\_IP` is the service cluster IP for the
API server as described in previous subsection.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.