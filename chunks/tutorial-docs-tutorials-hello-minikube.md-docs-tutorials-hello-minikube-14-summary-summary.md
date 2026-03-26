---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#14-summary
chunk_level: summary
chunk_type: prose
heading: Create a Deployment
token_count: 101
summary: 2. View the Deployment: ``` `kubectl get deployments ` ``` The output is similar to: ``` `NAME READY UP-TO-DATE AVAILABLE AGE hello-node 1/1 1 1 1m ` ``` (It may take some time for the pod to become...
---

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