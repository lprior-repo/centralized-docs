---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#20-summary
chunk_level: summary
chunk_type: prose
heading: Create a Service
token_count: 108
summary: ``` `kubectl expose deployment hello-node --type=LoadBalancer --port=8080 ` ``` The `--type=LoadBalancer` flag indicates that you want to expose your Service outside of the cluster. The application...
---

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