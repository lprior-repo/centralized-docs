---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#20-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 119
summary: ### Load Balancing Let's check that the Service is load-balancing the traffic. To find out the exposed IP and Port we can use `describe service` as we learned in the previous part of the tutorial:...
---

### Load Balancing
Let's check that the Service is load-balancing the traffic. To find out the exposed
IP and Port we can use `describe service` as we learned in the previous part of the tutorial:
```
`kubectl describe services/kubernetes-bootcamp
`
```
Create an environment variable called NODE\_PORT that has a value as the Node port:
```
`export NODE\_PORT="$(kubectl get services/kubernetes-bootcamp -o go-template='{{(index .spec.ports 0).nodePort}}')"
echo NODE\_PORT=$NODE\_PORT
`
```