---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#6-standard
chunk_level: standard
chunk_type: prose
heading: Example
token_count: 398
summary: ``` `pod/hello-apparmor-2 created ` ``` Although the Pod was created successfully, further examination will show that it is stuck in pending: ``` `kubectl describe pod hello-apparmor-2 ` ``` ```...
---

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