---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#21-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 69
summary: Next, we’ll do a `curl` to the exposed IP address and port. Execute the command multiple times: ``` `curl http://\"$(minikube ip):$NODE\_PORT\" ` ``` We hit a different Pod with every request. This...
---

Next, we’ll do a `curl` to the exposed IP address and port. Execute the command multiple times:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
We hit a different Pod with every request. This demonstrates that the load-balancing is working.
The output should be similar to: