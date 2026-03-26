---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#5-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 481
summary: ### Debugging Replication Controllers Replication controllers are fairly straightforward. They can either create Pods or they can't. If they can't create pods, then please refer to the [instructions...
---

### Debugging Replication Controllers
Replication controllers are fairly straightforward. They can either create Pods or they can't.
If they can't create pods, then please refer to the
[instructions above](#debugging-pods) to debug your pods.
You can also use `kubectl describe rc ${CONTROLLER\_NAME}` to introspect events
related to the replication controller.
### Debugging Services
Services provide load balancing across a set of pods. There are several common problems that can make Services
not work properly. The following instructions should help debug Service problems.
First, verify that there are endpoints for the service. For every Service object,
the apiserver makes one or more `EndpointSlice` resources available.
You can view these resources with:
```
`kubectl get endpointslices -l kubernetes.io/service-name=${SERVICE\_NAME}
`
```
Make sure that the endpoints in the EndpointSlices match up with the number of pods that you expect to be members of your service.
For example, if your Service is for an nginx container with 3 replicas, you would expect to see three different
IP addresses in the Service's endpoint slices.
#### My service is missing endpoints
If you are missing endpoints, try listing pods using the labels that Service uses.
Imagine that you have a Service where the labels are:
```
`...
spec:
- selector:
name: nginx
type: frontend
`
```
You can use:
```
`kubectl get pods --selector=name=nginx,type=frontend
`
```
to list pods that match this selector. Verify that the list matches the Pods that you expect to provide your Service.
Verify that the pod's `containerPort` matches up with the Service's `targetPort`
#### Network traffic is not forwarded
Please see [debugging service](/docs/tasks/debug/debug-application/debug-service/) for more information.
## What's next
If none of the above solves your problem, follow the instructions in
[Debugging Service document](/docs/tasks/debug/debug-application/debug-service/)
to make sure that your `Service` is running, has `Endpoints`, and your `Pods` are
actually serving; you have DNS working, iptables rules installed, and kube-proxy
does not seem to be misbehaving.
You may also visit [troubleshooting document](/docs/tasks/debug/) for more information.