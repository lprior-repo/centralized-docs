---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#15-summary
chunk_level: summary
chunk_type: prose
heading: Create a Deployment
token_count: 116
summary: ``` `kubectl get pods ` ``` The output is similar to: ``` `NAME READY STATUS RESTARTS AGE hello-node-5f76cf6ccf-br9b5 1/1 Running 0 1m ` ``` 4. View cluster events: ``` `kubectl get events ` ``` 5....
---

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