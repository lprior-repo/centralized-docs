---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#13-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 81
summary: ``` `curl http://localhost:8080/api/ ` ``` The output is similar to this: ``` `{ \"versions\": [ \"v1\" ], \"serverAddressByClientCIDRs\": [ { \"clientCIDR\": \"0.0.0.0/0\", \"serverAddress\": \"10.0.1.149:443\" }...
---

```
`curl http://localhost:8080/api/
`
```
The output is similar to this:
```
`{
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.1.149:443"
}
]
}
`
```