---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#2-standard
chunk_level: standard
chunk_type: code
heading: Adding additional entries with hostAliases
token_count: 398
summary: ## Adding additional entries with hostAliases In addition to the default boilerplate, you can add additional entries to the `hosts` file. For example: to resolve `foo.local`, `bar.local` to...
---

## Adding additional entries with hostAliases
In addition to the default boilerplate, you can add additional entries to the
`hosts` file.
For example: to resolve `foo.local`, `bar.local` to `127.0.0.1` and `foo.remote`,
`bar.remote` to `10.1.2.3`, you can configure HostAliases for a Pod under
`.spec.hostAliases`:
[`service/networking/hostaliases-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/hostaliases-pod.yaml)![](/images/copycode.svg "Copy service/networking/hostaliases-pod.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: hostaliases-pod
spec:
restartPolicy: Never
hostAliases:
- ip: "127.0.0.1"
hostnames:
- "foo.local"
- "bar.local"
- ip: "10.1.2.3"
hostnames:
- "foo.remote"
- "bar.remote"
containers:
- name: cat-hosts
image: busybox:1.28
command:
- cat
args:
- "/etc/hosts"
`
```
You can start a Pod with that configuration by running:
```
`kubectl apply -f https://k8s.io/examples/service/networking/hostaliases-pod.yaml
`
```
```
`pod/hostaliases-pod created
`
```
Examine a Pod's details to see its IPv4 address and its status:
```
`kubectl get pod --output=wide
`
```
```
`NAME READY STATUS RESTARTS AGE IP NODE
hostaliases-pod 0/1 Completed 0 6s 10.200.0.5 worker0
`
```
The `hosts` file content looks like this:
```
`kubectl logs hostaliases-pod
`
```