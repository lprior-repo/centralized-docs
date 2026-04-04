---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#19-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 112
summary: chmod +x cfssljson curl -L https://github.com/cloudflare/cfssl/releases/download/v1.5.0/cfssl-certinfo\_1.5.0\_linux\_amd64 -o cfssl-certinfo chmod +x cfssl-certinfo ` ``` 2. Create a directory to...
---

chmod +x cfssljson
curl -L https://github.com/cloudflare/cfssl/releases/download/v1.5.0/cfssl-certinfo\_1.5.0\_linux\_amd64 -o cfssl-certinfo
chmod +x cfssl-certinfo
`
```
2. Create a directory to hold the artifacts and initialize cfssl:
```
`mkdir cert
cd cert
../cfssl print-defaults config &gt; config.json
../cfssl print-defaults csr &gt; csr.json
`
```