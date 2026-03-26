---
doc_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods
chunk_id: ref/docs-tasks-debug-debug-application-debug-pods.md/docs-tasks-debug-debug-application-debug-pods#3-detailed
chunk_level: detailed
chunk_type: prose
heading: What's next
token_count: 899
summary: #### My pod is running but not doing what I told it to do If your pod is not behaving as you expected, it may be that there was an error in your pod description (e.g. `mypod.yaml` file on your local...
---

#### My pod is running but not doing what I told it to do
If your pod is not behaving as you expected, it may be that there was an error in your
pod description (e.g. `mypod.yaml` file on your local machine), and that the error
was silently ignored when you created the pod. Often a section of the pod description
is nested incorrectly, or a key name is typed incorrectly, and so the key is ignored.
For example, if you misspelled `command` as `commnd` then the pod will be created but
will not use the command line you intended it to use.
The first thing to do is to delete your pod and try creating it again with the `--validate` option.
For example, run `kubectl apply --validate -f mypod.yaml`.
If you misspelled `command` as `commnd` then will give an error like this:
```
`I0805 10:43:25.129850 46757 schema.go:126] unknown field: commnd
I0805 10:43:25.129973 46757 schema.go:129] this may be a false alarm, see https://github.com/kubernetes/kubernetes/issues/6842
pods/mypod
`
```
The next thing to check is whether the pod on the apiserver
matches the pod you meant to create (e.g. in a yaml file on your local machine).
For example, run `kubectl get pods/mypod -o yaml &gt; mypod-on-apiserver.yaml` and then
manually compare the original pod description, `mypod.yaml` with the one you got
back from apiserver, `mypod-on-apiserver.yaml`. There will typically be some
lines on the "apiserver" version that are not on the original version. This is
expected. However, if there are lines on the original that are not on the apiserver
version, then this may indicate a problem with your pod spec.
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