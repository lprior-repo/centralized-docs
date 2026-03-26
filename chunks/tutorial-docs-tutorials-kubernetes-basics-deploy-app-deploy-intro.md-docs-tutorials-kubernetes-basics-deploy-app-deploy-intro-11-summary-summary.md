---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#11-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 124
summary: *Applications need to be packaged into one of the supported container formats in order to be deployed on Kubernetes.* ![](/docs/tutorials/kubernetes-basics/public/images/module_02_first_app.svg) You...
---

*Applications need to be packaged into one of the supported container formats in
order to be deployed on Kubernetes.*
![](/docs/tutorials/kubernetes-basics/public/images/module_02_first_app.svg)
You can create and manage a Deployment by using the Kubernetes command line interface,
[kubectl](/docs/reference/kubectl/). `kubectl` uses the Kubernetes API to interact
with the cluster. In this module, you'll learn the most common `kubectl` commands
needed to create Deployments that run your applications on a Kubernetes cluster.
When you create a Deployment, you'll need to specify the container image for your