---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#2-detailed
chunk_level: detailed
chunk_type: code
heading: Create a Service
token_count: 954
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
#### Note:
Replace `hello-node-5f76cf6ccf-br9b5` in the `kubectl logs` command with the name of the pod from the `kubectl get pods` command output.
```
`kubectl logs hello-node-5f76cf6ccf-br9b5
`
```
The output is similar to:
```
`I0911 09:19:26.677397 1 log.go:195] Started HTTP server on port 8080
I0911 09:19:26.677586 1 log.go:195] Started UDP server on port 8081
`
```
#### Note:
For more information about `kubectl` commands, see the [kubectl overview](/docs/reference/kubectl/).
## Create a Service
By default, the Pod is only accessible by its internal IP address within the
Kubernetes cluster. To make the `hello-node` Container accessible from outside the
Kubernetes virtual network, you have to expose the Pod as a
Kubernetes [*Service*](/docs/concepts/services-networking/service/).
#### Warning:
The agnhost container has a `/shell` endpoint, which is useful for
debugging, but dangerous to expose to the public internet. Do not run this on an
internet-facing cluster, or a production cluster.
1. Expose the Pod to the public internet using the `kubectl expose` command:
```
`kubectl expose deployment hello-node --type=LoadBalancer --port=8080
`
```
The `--type=LoadBalancer` flag indicates that you want to expose your Service
outside of the cluster.
The application code inside the test image only listens on TCP port 8080. If you used
`kubectl expose` to expose a different port, clients could not connect to that other port.
2. View the Service you created:
```
`kubectl get services
`
```
The output is similar to:
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
hello-node LoadBalancer 10.108.144.78 &lt;pending&gt; 8080:30369/TCP 21s
kubernetes ClusterIP 10.96.0.1 &lt;none&gt; 443/TCP 23m
`
```
On cloud providers that support load balancers,
an external IP address would be provisioned to access the Service. On minikube,
the `LoadBalancer` type makes the Service accessible through the `minikube service`
command.
3. Run the following command:
```
`minikube service hello-node
`
```
This opens up a browser window that serves your app and shows the app's response.