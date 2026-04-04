---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#26-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 121
summary: \"ST\": \"&lt;state&gt;\", \"L\": \"&lt;city&gt;\", \"O\": \"&lt;organization&gt;\", \"OU\": \"&lt;organization unit&gt;\" }] } ` ``` 7. Generate the key and certificate for the API server, which are by default...
---

"ST": "&lt;state&gt;",
"L": "&lt;city&gt;",
"O": "&lt;organization&gt;",
"OU": "&lt;organization unit&gt;"
}]
}
`
```
7. Generate the key and certificate for the API server, which are by default
saved into file `server-key.pem` and `server.pem` respectively:
```
`../cfssl gencert -ca=ca.pem -ca-key=ca-key.pem \\
--config=ca-config.json -profile=kubernetes \\
server-csr.json | ../cfssljson -bare server
`
```