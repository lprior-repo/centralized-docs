---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#5-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 504
summary: ### Check for errors in the DNS pod Use the `kubectl logs` command to see logs for the DNS containers. For CoreDNS: ``` `kubectl logs --namespace=kube-system -l k8s-app=kube-dns ` ``` Here is an...
---

### Check for errors in the DNS pod
Use the `kubectl logs` command to see logs for the DNS containers.
For CoreDNS:
```
`kubectl logs --namespace=kube-system -l k8s-app=kube-dns
`
```
Here is an example of a healthy CoreDNS log:
```
`.:53
2018/08/15 14:37:17 [INFO] CoreDNS-1.2.2
2018/08/15 14:37:17 [INFO] linux/amd64, go1.10.3, 2e322f6
CoreDNS-1.2.2
linux/amd64, go1.10.3, 2e322f6
2018/08/15 14:37:17 [INFO] plugin/reload: Running configuration MD5 = 24e6c59e83ce706f07bcc82c31b1ea1c
`
```
See if there are any suspicious or unexpected messages in the logs.
### Is DNS service up?
Verify that the DNS service is up by using the `kubectl get service` command.
```
`kubectl get svc --namespace=kube-system
`
```
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
...
kube-dns ClusterIP 10.0.0.10 &lt;none&gt; 53/UDP,53/TCP 1h
...
`
```
#### Note:
The service name is `kube-dns` for both CoreDNS and kube-dns deployments.
If you have created the Service or in the case it should be created by default
but it does not appear, see
[debugging Services](/docs/tasks/debug/debug-application/debug-service/) for
more information.
### Are DNS endpoints exposed?
You can verify that DNS endpoints are exposed by using the `kubectl get endpointslice`
command.
```
`kubectl get endpointslice -l kubernetes.io/service-name=kube-dns --namespace=kube-system
`
```
```
`NAME ADDRESSTYPE PORTS ENDPOINTS AGE
kube-dns-zxoja IPv4 53 10.180.3.17,10.180.3.17 1h
`
```
If you do not see the endpoints, see the endpoints section in the
[debugging Services](/docs/tasks/debug/debug-application/debug-service/) documentation.