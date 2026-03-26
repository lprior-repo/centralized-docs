---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#15-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 125
summary: We should have 1 Pod. If not, run the command again. This shows: * *NAME* lists the names of the Deployments in the cluster. * *READY* shows the ratio of CURRENT/DESIRED replicas * *UP-TO-DATE*...
---

We should have 1 Pod. If not, run the command again. This shows:
* *NAME* lists the names of the Deployments in the cluster.
* *READY* shows the ratio of CURRENT/DESIRED replicas
* *UP-TO-DATE* displays the number of replicas that have been updated to achieve the desired state.
* *AVAILABLE* displays how many replicas of the application are available to your users.
* *AGE* displays the amount of time that the application has been running.
To see the ReplicaSet created by the Deployment, run:
```
`kubectl get rs
`
```