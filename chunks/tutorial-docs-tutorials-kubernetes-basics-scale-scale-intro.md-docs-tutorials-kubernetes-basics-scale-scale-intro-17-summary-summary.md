---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#17-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 121
summary: * *DESIRED* displays the desired number of replicas of the application, which you define when you create the Deployment. This is the desired state. * *CURRENT* displays how many replicas are...
---

* *DESIRED* displays the desired number of replicas of the application, which you
define when you create the Deployment. This is the desired state.
* *CURRENT* displays how many replicas are currently running.
Next, let’s scale the Deployment to 4 replicas. We’ll use the `kubectl scale` command,
followed by the Deployment type, name and desired number of instances:
```
`kubectl scale deployments/kubernetes-bootcamp --replicas=4
`
```
To list your Deployments once again, use `get deployments`:
```
`kubectl get deployments
`
```