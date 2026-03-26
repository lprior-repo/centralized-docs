---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#4-standard
chunk_level: standard
chunk_type: prose
heading: Create a Deployment
token_count: 159
summary: #### Note: Replace `hello-node-5f76cf6ccf-br9b5` in the `kubectl logs` command with the name of the pod from the `kubectl get pods` command output. ``` `kubectl logs hello-node-5f76cf6ccf-br9b5 ` ```...
---

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