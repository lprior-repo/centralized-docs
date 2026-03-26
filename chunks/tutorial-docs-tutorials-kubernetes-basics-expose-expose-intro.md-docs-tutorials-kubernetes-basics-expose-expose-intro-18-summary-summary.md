---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#18-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 58
summary: ``` `kubectl get services ` ``` To expose the deployment to external traffic, we'll use the kubectl expose command with the --type=NodePort option: ``` `kubectl expose deployment/kubernetes-bootcamp...
---

```
`kubectl get services
`
```
To expose the deployment to external traffic, we'll use the kubectl expose command with the --type=NodePort option:
```
`kubectl expose deployment/kubernetes-bootcamp --type="NodePort" --port 8080
`
```