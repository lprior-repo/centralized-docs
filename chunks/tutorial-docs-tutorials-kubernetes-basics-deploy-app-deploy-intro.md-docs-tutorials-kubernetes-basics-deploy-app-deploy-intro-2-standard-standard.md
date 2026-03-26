---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#2-standard
chunk_level: standard
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 506
summary: ## Deploying your first app on Kubernetes *Applications need to be packaged into one of the supported container formats in order to be deployed on Kubernetes.*...
---

## Deploying your first app on Kubernetes
*Applications need to be packaged into one of the supported container formats in
order to be deployed on Kubernetes.*
![](/docs/tutorials/kubernetes-basics/public/images/module_02_first_app.svg)
You can create and manage a Deployment by using the Kubernetes command line interface,
[kubectl](/docs/reference/kubectl/). `kubectl` uses the Kubernetes API to interact
with the cluster. In this module, you'll learn the most common `kubectl` commands
needed to create Deployments that run your applications on a Kubernetes cluster.
When you create a Deployment, you'll need to specify the container image for your
application and the number of replicas that you want to run. You can change that
information later by updating your Deployment; [Module 5](/docs/tutorials/kubernetes-basics/scale/scale-intro/)
and [Module 6](/docs/tutorials/kubernetes-basics/update/update-intro/) of the bootcamp
discuss how you can scale and update your Deployments.
For your first Deployment, you'll use a hello-node application packaged in a Docker
container that uses NGINX to echo back all the requests. (If you didn't already try
creating a hello-node application and deploying it using a container, you can do
that first by following the instructions from the [Hello Minikube tutorial](/docs/tutorials/hello-minikube/).)
You will need to have installed kubectl as well. If you need to install it, visit
[install tools](/docs/tasks/tools/#kubectl).
Now that you know what Deployments are, let's deploy our first app!
### kubectl basics
The common format of a kubectl command is: `kubectl action resource`.
This performs the specified *action* (like `create`, `describe` or `delete`) on the
specified *resource* (like `node` or `deployment`. You can use `--help` after the
subcommand to get additional info about possible parameters (for example: `kubectl get nodes --help`).
Check that kubectl is configured to talk to your cluster, by running the `kubectl version` command.
Check that kubectl is installed and that you can see both the client and the server versions.
To view the nodes in the cluster, run the `kubectl get nodes` command.
You see the available nodes. Later, Kubernetes will choose where to deploy our
application based on Node available resources.