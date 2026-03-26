---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#29-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 115
summary: ``` `curl http://\"$(minikube ip):$NODE\_PORT\" ` ``` This proves that the application is not reachable anymore from outside of the cluster. You can confirm that the app is still running with a `curl`...
---

```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
This proves that the application is not reachable anymore from outside of the cluster.
You can confirm that the app is still running with a `curl` from inside the pod:
```
`kubectl exec -ti $POD\_NAME -- curl http://localhost:8080
`
```
We see here that the application is up. This is because the Deployment is managing
the application. To shut down the application, you would need to delete the Deployment
as well.