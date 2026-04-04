---
doc_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event
chunk_id: tutorial/docs-tasks-configure-pod-container-attach-handler-lifecycle-event.md/docs-tasks-configure-pod-container-attach-handler-lifecycle-event#5-summary
chunk_level: summary
chunk_type: prose
heading: Define postStart and preStop handlers
token_count: 105
summary: ``` `apiVersion: v1 kind: Pod metadata: name: lifecycle-demo spec: containers: - name: lifecycle-demo-container image: nginx lifecycle: postStart: exec: command: [\"/bin/sh\", \"-c\", \"echo Hello from...
---

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