---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#9-summary
chunk_level: summary
chunk_type: prose
heading: Scaling an application
token_count: 52
summary: , first delete that Service and then run the following command to create a new Service with its `type` set to `LoadBalancer`: ``` `kubectl expose deployment/kubernetes-bootcamp --type=\"LoadBalancer\"...
---

,
first delete that Service and then run the following command to create a new Service
with its `type` set to `LoadBalancer`:
```
`kubectl expose deployment/kubernetes-bootcamp --type="LoadBalancer" --port 8080
`
```