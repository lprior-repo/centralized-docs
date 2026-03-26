---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#12-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 125
summary: needed to create Deployments that run your applications on a Kubernetes cluster. When you create a Deployment, you'll need to specify the container image for your application and the number of...
---

needed to create Deployments that run your applications on a Kubernetes cluster.
When you create a Deployment, you'll need to specify the container image for your
application and the number of replicas that you want to run. You can change that
information later by updating your Deployment; [Module 5](/docs/tutorials/kubernetes-basics/scale/scale-intro/)
and [Module 6](/docs/tutorials/kubernetes-basics/update/update-intro/) of the bootcamp
discuss how you can scale and update your Deployments.
For your first Deployment, you'll use a hello-node application packaged in a Docker