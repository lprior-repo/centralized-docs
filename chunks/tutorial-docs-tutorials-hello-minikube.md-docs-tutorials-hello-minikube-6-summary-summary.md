---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#6-summary
chunk_level: summary
chunk_type: prose
heading: Check the status of the minikube cluster
token_count: 118
summary: ## Create a minikube cluster ``` `minikube start ` ``` ## Check the status of the minikube cluster Verify the status of the minikube cluster to ensure all the components are in a running state. ```...
---

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