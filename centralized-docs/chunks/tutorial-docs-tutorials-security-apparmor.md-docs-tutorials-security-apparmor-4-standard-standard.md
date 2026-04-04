---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#4-standard
chunk_level: standard
chunk_type: code
heading: Example
token_count: 512
summary: ## Example *This example assumes you have already set up a cluster with AppArmor support.* First, load the profile you want to use onto your Nodes. This profile blocks all file write operations: ```...
---

## Example
*This example assumes you have already set up a cluster with AppArmor support.*
First, load the profile you want to use onto your Nodes. This profile blocks all file write operations:
```
`#include &lt;tunables/global&gt;
profile k8s-apparmor-example-deny-write flags=(attach\_disconnected) {
# Deny all file writes.
deny /\*\* w,
}
`
```
The profile needs to be loaded onto all nodes, since you don't know where the pod will be scheduled.
For this example you can use SSH to install the profiles, but other approaches are
discussed in [Setting up nodes with profiles](#setting-up-nodes-with-profiles).
```
`# This example assumes that node names match host names, and are reachable via SSH.
NODES=($( kubectl get node -o jsonpath='{.items[\*].status.addresses[?(.type == "Hostname")].address}' ))
for NODE in ${NODES[\*]}; do ssh $NODE 'sudo apparmor\_parser -q &lt;&lt;EOF
# Deny all file writes.
deny /\*\* w,
}
EOF'
done
`
```
Next, run a simple "Hello AppArmor" Pod with the deny-write profile:
[`pods/security/hello-apparmor.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/security/hello-apparmor.yaml)![](/images/copycode.svg "Copy pods/security/hello-apparmor.yaml to clipboard")
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