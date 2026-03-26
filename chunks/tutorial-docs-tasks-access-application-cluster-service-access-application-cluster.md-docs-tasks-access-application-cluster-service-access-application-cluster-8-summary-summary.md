---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 104
summary: ``` `kubectl get deployments hello-world kubectl describe deployments hello-world ` ``` 3. Display information about your ReplicaSet objects: ``` `kubectl get replicasets kubectl describe replicasets...
---

```
`kubectl get deployments hello-world
kubectl describe deployments hello-world
`
```
3. Display information about your ReplicaSet objects:
```
`kubectl get replicasets
kubectl describe replicasets
`
```
4. Create a Service object that exposes the deployment:
```
`kubectl expose deployment hello-world --type=NodePort --name=example-service
`
```
5. Display information about the Service:
```
`kubectl describe services example-service
`
```
The output is similar to this: