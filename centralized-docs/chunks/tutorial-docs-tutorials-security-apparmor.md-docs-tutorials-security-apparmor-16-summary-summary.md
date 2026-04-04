---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#16-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 108
summary: ``` `apiVersion: v1 kind: Pod metadata: name: hello-apparmor spec: securityContext: appArmorProfile: type: Localhost localhostProfile: k8s-apparmor-example-deny-write containers: - name: hello image:...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: hello-apparmor
spec:
securityContext:
appArmorProfile:
type: Localhost
localhostProfile: k8s-apparmor-example-deny-write
containers:
- name: hello
image: busybox:1.28
command: [ "sh", "-c", "echo 'Hello AppArmor!' &amp;&amp; sleep 1h" ]
`
```
```
`kubectl create -f hello-apparmor.yaml
`
```