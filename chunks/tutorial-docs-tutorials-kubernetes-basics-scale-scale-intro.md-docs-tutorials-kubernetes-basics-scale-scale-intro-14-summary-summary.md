---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#14-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 87
summary: ### Scaling a Deployment To list your Deployments, use the `get deployments` subcommand: ``` `kubectl get deployments ` ``` The output should be similar to: ``` `NAME READY UP-TO-DATE AVAILABLE AGE...
---

### Scaling a Deployment
To list your Deployments, use the `get deployments` subcommand:
```
`kubectl get deployments
`
```
The output should be similar to:
```
`NAME READY UP-TO-DATE AVAILABLE AGE
kubernetes-bootcamp 1/1 1 1 11m
`
```
We should have 1 Pod. If not, run the command again. This shows: