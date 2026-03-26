---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Open the Dashboard
token_count: 607
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
## Open the Dashboard
Open the Kubernetes dashboard. You can do this two different ways:
Open a **new** terminal, and run:
```
`# Start a new terminal, and leave this running.
minikube dashboard
`
```
Now, switch back to the terminal where you ran `minikube start`.
#### Note:
The `dashboard` command enables the dashboard add-on and opens the proxy in the default web browser.
You can create Kubernetes resources on the dashboard such as Deployment and Service.
To find out how to avoid directly invoking the browser from the terminal and get a URL for the web dashboard, see the "URL copy and paste" tab.
By default, the dashboard is only accessible from within the internal Kubernetes virtual network.
The `dashboard` command creates a temporary proxy to make the dashboard accessible from outside the Kubernetes virtual network.
To stop the proxy, run `Ctrl+C` to exit the process.
After the command exits, the dashboard remains running in the Kubernetes cluster.
You can run the `dashboard` command again to create another proxy to access the dashboard.
If you don't want minikube to open a web browser for you, run the `dashboard` subcommand with the
`--url` flag. `minikube` outputs a URL that you can open in the browser you prefer.
Open a **new** terminal, and run:
```
`# Start a new terminal, and leave this running.
minikube dashboard --url
`
```
Now, you can use this URL and switch back to the terminal where you ran `minikube start`.