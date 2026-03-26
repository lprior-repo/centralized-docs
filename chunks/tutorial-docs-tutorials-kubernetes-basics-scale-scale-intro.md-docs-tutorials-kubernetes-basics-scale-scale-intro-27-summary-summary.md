---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#27-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 114
summary: ### Scale Down To scale down the Deployment to 2 replicas, run again the `scale` subcommand: ``` `kubectl scale deployments/kubernetes-bootcamp --replicas=2 ` ``` List the Deployments to check if the...
---

### Scale Down
To scale down the Deployment to 2 replicas, run again the `scale` subcommand:
```
`kubectl scale deployments/kubernetes-bootcamp --replicas=2
`
```
List the Deployments to check if the change was applied with the `get deployments` subcommand:
```
`kubectl get deployments
`
```
The number of replicas decreased to 2. List the number of Pods, with `get pods`:
```
`kubectl get pods -o wide
`
```
This confirms that 2 Pods were terminated.