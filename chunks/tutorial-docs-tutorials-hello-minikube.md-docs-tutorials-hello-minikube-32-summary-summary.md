---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#32-summary
chunk_level: summary
chunk_type: prose
heading: Conclusion
token_count: 128
summary: ## Clean up Now you can clean up the resources you created in your cluster: ``` `kubectl delete service hello-node kubectl delete deployment hello-node ` ``` Stop the Minikube cluster ``` `minikube...
---

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