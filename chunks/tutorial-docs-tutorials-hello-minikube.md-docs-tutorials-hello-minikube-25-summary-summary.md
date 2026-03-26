---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#25-summary
chunk_level: summary
chunk_type: prose
heading: Enable addons
token_count: 85
summary: 2. Enable an addon, for example, `metrics-server`: ``` `minikube addons enable metrics-server ` ``` The output is similar to: ``` `The 'metrics-server' addon is enabled ` ``` 3. View the Pod and...
---

2. Enable an addon, for example, `metrics-server`:
```
`minikube addons enable metrics-server
`
```
The output is similar to:
```
`The 'metrics-server' addon is enabled
`
```
3. View the Pod and Service you created by installing that addon:
```
`kubectl get pod,svc -n kube-system
`
```
The output is similar to: