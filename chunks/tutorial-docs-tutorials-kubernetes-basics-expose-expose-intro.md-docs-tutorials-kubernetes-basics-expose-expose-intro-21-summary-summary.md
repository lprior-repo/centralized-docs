---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#21-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 49
summary: Now we can test that the app is exposed outside of the cluster using `curl`, the IP address of the Node and the externally exposed port: ``` `curl http://\"$(minikube ip):$NODE\_PORT\" ` ```
---

Now we can test that the app is exposed outside of the cluster using `curl`, the
IP address of the Node and the externally exposed port:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```