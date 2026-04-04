---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Example
token_count: 678
summary: To wrap up, see what happens if you try to specify a profile that hasn't been loaded: ``` `kubectl create -f /dev/stdin &lt;&lt;EOF apiVersion: v1 kind: Pod metadata: name: hello-apparmor-2 spec:...
---

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
```
`Name: hello-apparmor-2
Namespace: default
Node: gke-test-default-pool-239f5d02-x1kf/10.128.0.27
Start Time: Tue, 30 Aug 2016 17:58:56 -0700
Labels: &lt;none&gt;
Annotations: container.apparmor.security.beta.kubernetes.io/hello=localhost/k8s-apparmor-example-allow-write
Status: Pending
...
Events:
Type Reason Age From Message
---- ------ ---- ---- -------
Normal Scheduled 10s default-scheduler Successfully assigned default/hello-apparmor to gke-test-default-pool-239f5d02-x1kf
Normal Pulled 8s kubelet Successfully pulled image "busybox:1.28" in 370.157088ms (370.172701ms including waiting)
Normal Pulling 7s (x2 over 9s) kubelet Pulling image "busybox:1.28"
Warning Failed 7s (x2 over 8s) kubelet Error: failed to get container spec opts: failed to generate apparmor spec opts: apparmor profile not found k8s-apparmor-example-allow-write
Normal Pulled 7s kubelet Successfully pulled image "busybox:1.28" in 90.980331ms (91.005869ms including waiting)
`
```
An Event provides the error message with the reason, the specific wording is runtime-dependent:
```
` Warning Failed 7s (x2 over 8s) kubelet Error: failed to get container spec opts: failed to generate apparmor spec opts: apparmor profile not found
`
```
### Setting up Nodes with profiles
Kubernetes 1.35 does not provide any built-in mechanisms for loading AppArmor profiles onto
Nodes. Profiles can be loaded through custom infrastructure or tools like the
[Kubernetes Security Profiles Operator](https://github.com/kubernetes-sigs/security-profiles-operator).
The scheduler is not aware of which profiles are loaded onto which Node, so the full set of profiles
must be loaded onto every Node. An alternative approach is to add a Node label for each profile (or
class of profiles) on the Node, and use a
[node selector](/docs/concepts/scheduling-eviction/assign-pod-node/) to ensure the Pod is run on a
Node with the required profile.