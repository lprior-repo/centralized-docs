---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#25-summary
chunk_level: summary
chunk_type: prose
heading: Diagnosing the problem
token_count: 82
summary: ``` `kubectl get endpointslices -l kubernetes.io/service-name=${SERVICE\_NAME} ` ``` Make sure that the endpoints in the EndpointSlices match up with the number of pods that you expect to be members...
---

```
`kubectl get endpointslices -l kubernetes.io/service-name=${SERVICE\_NAME}
`
```
Make sure that the endpoints in the EndpointSlices match up with the number of pods that you expect to be members of your service.
For example, if your Service is for an nginx container with 3 replicas, you would expect to see three different
IP addresses in the Service's endpoint slices.