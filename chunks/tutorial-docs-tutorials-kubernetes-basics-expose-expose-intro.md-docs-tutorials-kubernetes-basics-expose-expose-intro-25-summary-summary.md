---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#25-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 111
summary: ``` `kubectl get pods -l app=kubernetes-bootcamp ` ``` You can do the same to list the existing Services: ``` `kubectl get services -l app=kubernetes-bootcamp ` ``` Get the name of the Pod and store...
---

```
`kubectl get pods -l app=kubernetes-bootcamp
`
```
You can do the same to list the existing Services:
```
`kubectl get services -l app=kubernetes-bootcamp
`
```
Get the name of the Pod and store it in the POD\_NAME environment variable:
```
`export POD\_NAME="$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\\n"}}{{end}}')"
echo "Name of the Pod: $POD\_NAME"
`
```