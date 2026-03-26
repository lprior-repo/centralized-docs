---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#22-summary
chunk_level: summary
chunk_type: prose
heading: Create a Service
token_count: 84
summary: On cloud providers that support load balancers, an external IP address would be provisioned to access the Service. On minikube, the `LoadBalancer` type makes the Service accessible through the...
---

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