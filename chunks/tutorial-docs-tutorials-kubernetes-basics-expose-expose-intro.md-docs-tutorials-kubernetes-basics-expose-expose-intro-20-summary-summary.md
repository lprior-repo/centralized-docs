---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#20-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 110
summary: ``` `kubectl describe services/kubernetes-bootcamp ` ``` Create an environment variable called `NODE\_PORT` that has the value of the Node port assigned: ``` `export NODE\_PORT=\"$(kubectl get...
---

```
`kubectl describe services/kubernetes-bootcamp
`
```
Create an environment variable called `NODE\_PORT` that has the value of the Node
port assigned:
```
`export NODE\_PORT="$(kubectl get services/kubernetes-bootcamp -o go-template='{{(index .spec.ports 0).nodePort}}')"
echo "NODE\_PORT=$NODE\_PORT"
`
```
Now we can test that the app is exposed outside of the cluster using `curl`, the
IP address of the Node and the externally exposed port: