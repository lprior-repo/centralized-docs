---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#8-standard
chunk_level: standard
chunk_type: code
heading: What's next
token_count: 350
summary: 4. Check the output from `metrics-server`: ``` `kubectl top pods ` ``` The output is similar to: ``` `NAME CPU(cores) MEMORY(bytes) hello-node-ccf4b9788-4jn97 1m 6Mi ` ``` If you see the following...
---

4. Check the output from `metrics-server`:
```
`kubectl top pods
`
```
The output is similar to:
```
`NAME CPU(cores) MEMORY(bytes)
hello-node-ccf4b9788-4jn97 1m 6Mi
`
```
If you see the following message, wait, and try again:
```
`error: Metrics API not available
`
```
5. Disable `metrics-server`:
```
`minikube addons disable metrics-server
`
```
The output is similar to:
```
`metrics-server was successfully disabled
`
```
## Clean up
Now you can clean up the resources you created in your cluster:
```
`kubectl delete service hello-node
kubectl delete deployment hello-node
`
```
Stop the Minikube cluster
```
`minikube stop
`
```
Optionally, delete the Minikube VM:
```
`# Optional
minikube delete
`
```
If you want to use minikube again to learn more about Kubernetes, you don't need to delete it.
## Conclusion
This page covered the basic aspects to get a minikube cluster up and running. You are now ready to deploy applications.
## What's next
* Tutorial to *[deploy your first app on Kubernetes with kubectl](/docs/tutorials/kubernetes-basics/deploy-app/deploy-intro/)*.
* Learn more about [Deployment objects](/docs/concepts/workloads/controllers/deployment/).
* Learn more about [Deploying applications](/docs/tasks/run-application/run-stateless-application-deployment/).
* Learn more about [Service objects](/docs/concepts/services-networking/service/).