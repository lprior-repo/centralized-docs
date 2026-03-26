---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#31-summary
chunk_level: summary
chunk_type: prose
heading: Enable addons
token_count: 126
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