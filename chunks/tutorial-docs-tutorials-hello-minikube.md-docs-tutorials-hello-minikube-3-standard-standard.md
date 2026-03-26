---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#3-standard
chunk_level: standard
chunk_type: code
heading: Create a Deployment
token_count: 391
summary: ## Create a Deployment A Kubernetes [*Pod*](/docs/concepts/workloads/pods/) is a group of one or more Containers, tied together for the purposes of administration and networking. The Pod in this...
---

## Create a Deployment
A Kubernetes [*Pod*](/docs/concepts/workloads/pods/) is a group of one or more Containers,
tied together for the purposes of administration and networking. The Pod in this
tutorial has only one Container. A Kubernetes
[*Deployment*](/docs/concepts/workloads/controllers/deployment/) checks on the health of your
Pod and restarts the Pod's Container if it terminates. Deployments are the
recommended way to manage the creation and scaling of Pods.
1. Use the `kubectl create` command to create a Deployment that manages a Pod. The
Pod runs a Container based on the provided Docker image.
```
`# Run a test container image that includes a webserver
kubectl create deployment hello-node --image=registry.k8s.io/e2e-test-images/agnhost:2.53 -- /agnhost netexec --http-port=8080
`
```
2. View the Deployment:
```
`kubectl get deployments
`
```
The output is similar to:
```
`NAME READY UP-TO-DATE AVAILABLE AGE
hello-node 1/1 1 1 1m
`
```
(It may take some time for the pod to become available. If you see "0/1", try again in a few seconds.)
3. View the Pod:
```
`kubectl get pods
`
```
The output is similar to:
```
`NAME READY STATUS RESTARTS AGE
hello-node-5f76cf6ccf-br9b5 1/1 Running 0 1m
`
```
4. View cluster events:
```
`kubectl get events
`
```
5. View the `kubectl` configuration:
```
`kubectl config view
`
```
6. View application logs for a container in a pod (replace pod name with the one you got from `kubectl get pods`).