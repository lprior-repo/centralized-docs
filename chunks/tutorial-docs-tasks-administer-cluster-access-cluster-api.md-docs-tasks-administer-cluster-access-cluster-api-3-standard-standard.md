---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#3-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 357
summary: ### Directly accessing the REST API kubectl handles locating and authenticating to the API server. If you want to directly access the REST API with an http client like `curl` or `wget`, or a browser,...
---

### Directly accessing the REST API
kubectl handles locating and authenticating to the API server. If you want to directly access the REST API with an http client like
`curl` or `wget`, or a browser, there are multiple ways you can locate and authenticate against the API server:
1. Run kubectl in proxy mode (recommended). This method is recommended, since it uses
the stored API server location and verifies the identity of the API server using a
self-signed certificate. No man-in-the-middle (MITM) attack is possible using this method.
2. Alternatively, you can provide the location and credentials directly to the http client.
This works with client code that is confused by proxies. To protect against man in the
middle attacks, you'll need to import a root cert into your browser.
Using the Go or Python client libraries provides accessing kubectl in proxy mode.
#### Using kubectl proxy
The following command runs kubectl in a mode where it acts as a reverse proxy. It handles
locating the API server and authenticating.
Run it like this:
```
`kubectl proxy --port=8080 &amp;
`
```
See [kubectl proxy](/docs/reference/generated/kubectl/kubectl-commands/#proxy) for more details.
Then you can explore the API with curl, wget, or a browser, like so:
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