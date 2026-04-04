---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#5-standard
chunk_level: standard
chunk_type: code
heading: Example
token_count: 322
summary: You can verify that the container is actually running with that profile by checking `/proc/1/attr/current`: ``` `kubectl exec hello-apparmor -- cat /proc/1/attr/current ` ``` The output should be:...
---

You can verify that the container is actually running with that profile by checking `/proc/1/attr/current`:
```
`kubectl exec hello-apparmor -- cat /proc/1/attr/current
`
```
The output should be:
```
`k8s-apparmor-example-deny-write (enforce)
`
```
Finally, you can see what happens if you violate the profile by writing to a file:
```
`kubectl exec hello-apparmor -- touch /tmp/test
`
```
```
`touch: /tmp/test: Permission denied
error: error executing remote command: command terminated with non-zero exit code: Error executing in Docker Container: 1
`
```
To wrap up, see what happens if you try to specify a profile that hasn't been loaded:
```
`kubectl create -f /dev/stdin &lt;&lt;EOF
apiVersion: v1
kind: Pod
metadata:
name: hello-apparmor-2
spec:
securityContext:
appArmorProfile:
type: Localhost
localhostProfile: k8s-apparmor-example-allow-write
containers:
- name: hello
image: busybox:1.28
command: [ "sh", "-c", "echo 'Hello AppArmor!' &amp;&amp; sleep 1h" ]
EOF
`
```
```
`pod/hello-apparmor-2 created
`
```
Although the Pod was created successfully, further examination will show that it is stuck in pending:
```
`kubectl describe pod hello-apparmor-2
`
```