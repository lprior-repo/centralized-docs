---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#19-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 122
summary: Great! You just deployed your first application by creating a deployment. This performed a few things for you: * searched for a suitable node where an instance of the application could be run (we...
---

Great! You just deployed your first application by creating a deployment. This performed a few things for you:
* searched for a suitable node where an instance of the application could be run (we have only 1 available node)
* scheduled the application to run on that Node
* configured the cluster to reschedule the instance on a new Node when needed
To list your deployments use the `kubectl get deployments` command:
```
`kubectl get deployments
`
```
We see that there is 1 deployment running a single instance of your app. The instance
is running inside a container on your node.