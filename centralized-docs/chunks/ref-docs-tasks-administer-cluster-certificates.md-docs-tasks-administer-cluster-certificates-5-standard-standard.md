---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#5-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 445
summary: ``` `{ \"CN\": \"kubernetes\", \"key\": { \"algo\": \"rsa\", \"size\": 2048 }, \"names\":[{ \"C\": \"&lt;country&gt;\", \"ST\": \"&lt;state&gt;\", \"L\": \"&lt;city&gt;\", \"O\": \"&lt;organization&gt;\", \"OU\": \"&lt;organization...
---

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
5. Generate CA key (`ca-key.pem`) and certificate (`ca.pem`):
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
```
`{
"CN": "kubernetes",
"hosts": [
"127.0.0.1",
"&lt;MASTER\_IP&gt;",
"&lt;MASTER\_CLUSTER\_IP&gt;",
"kubernetes",
"kubernetes.default",
"kubernetes.default.svc",
"kubernetes.default.svc.cluster",
"kubernetes.default.svc.cluster.local"
],
"key": {
"algo": "rsa",
"size": 2048
},
"names": [{
"C": "&lt;country&gt;",
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