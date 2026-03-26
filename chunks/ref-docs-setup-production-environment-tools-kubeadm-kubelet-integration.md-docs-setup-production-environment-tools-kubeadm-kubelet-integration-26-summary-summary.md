---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#26-summary
chunk_level: summary
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 121
summary: ``` `KUBELET\_KUBEADM\_ARGS=\"--flag1=value1 --flag2=value2 ...\" ` ``` In addition to the flags used when starting the kubelet, the file also contains dynamic parameters such as the cgroup driver....
---

```
`KUBELET\_KUBEADM\_ARGS="--flag1=value1 --flag2=value2 ..."
`
```
In addition to the flags used when starting the kubelet, the file also contains dynamic
parameters such as the cgroup driver.
After marshalling these two files to disk, kubeadm attempts to run the following two
commands, if you are using systemd:
```
`systemctl daemon-reload &amp;&amp; systemctl restart kubelet
`
```
If the reload and restart are successful, the normal `kubeadm init` workflow continues.