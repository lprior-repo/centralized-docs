---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#6-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 123
summary: ``` `apiVersion: apps/v1 kind: Deployment metadata: name: hello-world spec: selector: matchLabels: run: load-balancer-example replicas: 2 template: metadata: labels: run: load-balancer-example spec:...
---

```
`apiVersion: apps/v1
kind: Deployment
metadata:
name: hello-world
spec:
selector:
matchLabels:
run: load-balancer-example
replicas: 2
template:
metadata:
labels:
run: load-balancer-example
spec:
containers:
- name: hello-world
image: us-docker.pkg.dev/google-samples/containers/gke/hello-app:2.0
ports:
- containerPort: 8080
protocol: TCP
`
```
1. Run a Hello World application in your cluster:
Create the application Deployment using the file above: