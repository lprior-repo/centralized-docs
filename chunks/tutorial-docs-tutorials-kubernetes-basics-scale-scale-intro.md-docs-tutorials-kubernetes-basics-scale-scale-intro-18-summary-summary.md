---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#18-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 112
summary: To list your Deployments once again, use `get deployments`: ``` `kubectl get deployments ` ``` The change was applied, and we have 4 instances of the application available. Next, let’s check if the...
---

To list your Deployments once again, use `get deployments`:
```
`kubectl get deployments
`
```
The change was applied, and we have 4 instances of the application available. Next,
let’s check if the number of Pods changed:
```
`kubectl get pods -o wide
`
```
There are 4 Pods now, with different IP addresses. The change was registered in
the Deployment events log. To check that, use the `describe` subcommand:
```
`kubectl describe deployments/kubernetes-bootcamp
`
```