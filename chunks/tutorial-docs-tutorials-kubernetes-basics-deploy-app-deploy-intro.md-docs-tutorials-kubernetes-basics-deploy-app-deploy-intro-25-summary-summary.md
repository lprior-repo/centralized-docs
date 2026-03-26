---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#25-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 95
summary: ``` `export POD\_NAME=$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{\"\\n\"}}{{end}}') echo Name of the Pod: $POD\_NAME ` ``` You can access the Pod through the...
---

```
`export POD\_NAME=$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\\n"}}{{end}}')
echo Name of the Pod: $POD\_NAME
`
```
You can access the Pod through the proxied API, by running:
```
`curl http://localhost:8001/api/v1/namespaces/default/pods/$POD\_NAME:8080/proxy/
`
```