---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#20-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 89
summary: 3. Create a JSON config file for generating the CA file, for example, `ca-config.json`: ``` `{ \"signing\": { \"default\": { \"expiry\": \"8760h\" }, \"profiles\": { \"kubernetes\": { \"usages\": [ \"signing\", \"key...
---

3. Create a JSON config file for generating the CA file, for example, `ca-config.json`:
```
`{
"signing": {
"default": {
"expiry": "8760h"
},
"profiles": {
"kubernetes": {
"usages": [
"signing",
"key encipherment",
"server auth",
"client auth"
],
"expiry": "8760h"
}
}
}
}
`
```