---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#3-standard
chunk_level: standard
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 196
summary: ### Deploy an app Let’s deploy our first app on Kubernetes with the `kubectl create deployment` command. We need to provide the deployment name and app image location (include the full repository url...
---

### Deploy an app
Let’s deploy our first app on Kubernetes with the `kubectl create deployment` command.
We need to provide the deployment name and app image location (include the full
repository url for images hosted outside Docker Hub).
```
`kubectl create deployment kubernetes-bootcamp --image=gcr.io/google-samples/kubernetes-bootcamp:v1
`
```
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