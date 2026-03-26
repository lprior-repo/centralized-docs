---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#1-standard
chunk_level: standard
chunk_type: prose
heading: Check the status of the minikube cluster
token_count: 281
summary: # Hello Minikube This tutorial shows you how to run a sample app on Kubernetes using minikube. The tutorial provides a container image that uses NGINX to echo back all the requests. ## Objectives *...
---

# Hello Minikube
This tutorial shows you how to run a sample app on Kubernetes using minikube.
The tutorial provides a container image that uses NGINX to echo back all the requests.
## Objectives
* Deploy a sample application to minikube.
* Run the app.
* View application logs.## Before you begin
This tutorial assumes that you have already set up `minikube`.
See **Step 1** in [minikube start](https://minikube.sigs.k8s.io/docs/start/) for installation instructions.
#### Note:
Only execute the instructions in **Step 1, Installation**. The rest is covered on this page.
You also need to install `kubectl`.
See [Install tools](/docs/tasks/tools/#kubectl) for installation instructions.
## Create a minikube cluster
```
`minikube start
`
```
## Check the status of the minikube cluster
Verify the status of the minikube cluster to ensure all the components are in a running state.
```
`minikube status
`
```
The output from the above command should show all components Running or Configured, as shown in the example output below:
```
`minikube
type: Control Plane
host: Running
kubelet: Running
apiserver: Running
kubeconfig: Configured
`
```