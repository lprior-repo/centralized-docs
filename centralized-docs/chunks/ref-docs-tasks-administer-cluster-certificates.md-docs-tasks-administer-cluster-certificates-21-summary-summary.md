---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#21-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 123
summary: 4. Create a JSON config file for CA certificate signing request (CSR), for example, `ca-csr.json`. Be sure to replace the values marked with angle brackets with real values you want to use. ``` `{...
---

4. Create a JSON config file for CA certificate signing request (CSR), for example,
`ca-csr.json`. Be sure to replace the values marked with angle brackets with
real values you want to use.
```
`{
"CN": "kubernetes",
"key": {
"algo": "rsa",
"size": 2048
},
"names":[{
"C": "&lt;country&gt;",
"ST": "&lt;state&gt;",
"L": "&lt;city&gt;",
"O": "&lt;organization&gt;",
"OU": "&lt;organization unit&gt;"
}]
}
`
```