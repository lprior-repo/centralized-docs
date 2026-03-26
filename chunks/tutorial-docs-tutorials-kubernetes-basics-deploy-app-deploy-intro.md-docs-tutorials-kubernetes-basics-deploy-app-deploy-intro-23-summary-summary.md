---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#23-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 84
summary: ``` `kubectl proxy ` ``` We now have a connection between our host (the terminal) and the Kubernetes cluster. The proxy enables direct access to the API from these terminals. You can see all those...
---

```
`kubectl proxy
`
```
We now have a connection between our host (the terminal) and the Kubernetes cluster.
The proxy enables direct access to the API from these terminals.
You can see all those APIs hosted through the proxy endpoint. For example, we can
query the version directly through the API using the `curl` command:
```
`curl http://localhost:8001/version
`
```