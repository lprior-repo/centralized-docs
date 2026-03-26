---
doc_id: tutorial/docs-tasks-extend-kubernetes-http-proxy-access-api.md/docs-tasks-extend-kubernetes-http-proxy-access-api
chunk_id: tutorial/docs-tasks-extend-kubernetes-http-proxy-access-api.md/docs-tasks-extend-kubernetes-http-proxy-access-api#6-summary
chunk_level: summary
chunk_type: prose
heading: Exploring the Kubernetes API
token_count: 125
summary: ## Exploring the Kubernetes API When the proxy server is running, you can explore the API using `curl`, `wget`, or a browser. Get the API versions: ``` `curl http://localhost:8080/api/ ` ``` The...
---

## Exploring the Kubernetes API
When the proxy server is running, you can explore the API using `curl`, `wget`,
or a browser.
Get the API versions:
```
`curl http://localhost:8080/api/
`
```
The output should look similar to this:
```
`{
"kind": "APIVersions",
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.2.15:8443"
}
]
}
`
```