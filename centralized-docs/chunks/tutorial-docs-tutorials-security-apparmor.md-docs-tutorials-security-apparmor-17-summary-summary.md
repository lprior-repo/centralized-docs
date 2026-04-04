---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#17-summary
chunk_level: summary
chunk_type: prose
heading: Example
token_count: 117
summary: ``` `kubectl create -f hello-apparmor.yaml ` ``` You can verify that the container is actually running with that profile by checking `/proc/1/attr/current`: ``` `kubectl exec hello-apparmor -- cat...
---

```
`kubectl create -f hello-apparmor.yaml
`
```
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