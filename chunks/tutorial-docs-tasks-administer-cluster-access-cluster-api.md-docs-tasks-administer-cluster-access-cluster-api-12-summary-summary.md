---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#12-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 118
summary: #### Using kubectl proxy The following command runs kubectl in a mode where it acts as a reverse proxy. It handles locating the API server and authenticating. Run it like this: ``` `kubectl proxy...
---

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