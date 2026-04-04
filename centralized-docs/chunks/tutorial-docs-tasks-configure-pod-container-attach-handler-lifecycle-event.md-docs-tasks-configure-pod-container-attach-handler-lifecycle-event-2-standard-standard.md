---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#2-standard
chunk_level: standard
chunk_type: code
heading: Define postStart and preStop handlers
token_count: 389
summary: ## Define postStart and preStop handlers In this exercise, you create a Pod that has one Container. The Container has handlers for the postStart and preStop events. Here is the configuration file for...
---

## Define postStart and preStop handlers
In this exercise, you create a Pod that has one Container. The Container has handlers
for the postStart and preStop events.
Here is the configuration file for the Pod:
[`pods/lifecycle-events.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/lifecycle-events.yaml)![](/images/copycode.svg "Copy pods/lifecycle-events.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: lifecycle-demo
spec:
containers:
- name: lifecycle-demo-container
image: nginx
lifecycle:
postStart:
exec:
command: ["/bin/sh", "-c", "echo Hello from the postStart handler &gt; /usr/share/message"]
preStop:
exec:
command: ["/bin/sh","-c","nginx -s quit; while killall -0 nginx; do sleep 1; done"]
`
```
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
```
`kubectl exec -it lifecycle-demo -- /bin/bash
`
```
In your shell, verify that the `postStart` handler created the `message` file:
```
`root@lifecycle-demo:/# cat /usr/share/message
`
```
The output shows the text written by the postStart handler:
```
`Hello from the postStart handler
`
```