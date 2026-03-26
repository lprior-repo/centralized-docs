---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#19-summary
chunk_level: summary
chunk_type: prose
heading: Create a Service
token_count: 87
summary: #### Warning: The agnhost container has a `/shell` endpoint, which is useful for debugging, but dangerous to expose to the public internet. Do not run this on an internet-facing cluster, or a...
---

#### Warning:
The agnhost container has a `/shell` endpoint, which is useful for
debugging, but dangerous to expose to the public internet. Do not run this on an
internet-facing cluster, or a production cluster.
1. Expose the Pod to the public internet using the `kubectl expose` command:
```
`kubectl expose deployment hello-node --type=LoadBalancer --port=8080
`
```