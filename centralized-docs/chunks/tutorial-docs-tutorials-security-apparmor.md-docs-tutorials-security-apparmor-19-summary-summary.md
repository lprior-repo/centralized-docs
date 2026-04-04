---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#19-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 127
summary: ``` `kubectl create -f /dev/stdin &lt;&lt;EOF apiVersion: v1 kind: Pod metadata: name: hello-apparmor-2 spec: securityContext: appArmorProfile: type: Localhost localhostProfile:...
---

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