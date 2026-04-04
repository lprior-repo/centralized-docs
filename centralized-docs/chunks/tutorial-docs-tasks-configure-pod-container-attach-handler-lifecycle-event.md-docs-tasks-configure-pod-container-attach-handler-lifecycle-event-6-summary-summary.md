---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#6-summary
chunk_level: summary
chunk_type: prose
heading: Define postStart and preStop handlers
token_count: 114
summary: In the configuration file, you can see that the postStart command writes a `message` file to the Container's `/usr/share` directory. The preStop command shuts down nginx gracefully. This is helpful...
---

In the configuration file, you can see that the postStart command writes a `message`
file to the Container's `/usr/share` directory. The preStop command shuts down
nginx gracefully. This is helpful if the Container is being terminated because of a failure.
Create the Pod:
```
`kubectl apply -f https://k8s.io/examples/pods/lifecycle-events.yaml
`
```
Verify that the Container in the Pod is running:
```
`kubectl get pod lifecycle-demo
`
```
Get a shell into the Container running in your Pod: