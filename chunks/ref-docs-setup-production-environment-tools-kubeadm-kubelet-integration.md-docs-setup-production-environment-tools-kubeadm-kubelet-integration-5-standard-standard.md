---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#5-standard
chunk_level: standard
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 388
summary: ### Workflow when using `kubeadm init` When you call `kubeadm init`, the kubelet configuration is marshalled to disk at `/var/lib/kubelet/config.yaml`, and also uploaded to a `kubelet-config`...
---

### Workflow when using `kubeadm init`
When you call `kubeadm init`, the kubelet configuration is marshalled to disk
at `/var/lib/kubelet/config.yaml`, and also uploaded to a `kubelet-config`
ConfigMap in the `kube-system` namespace of the cluster.
Additionally, the kubeadm tool detects the CRI socket on the node and writes its details
(including the socket path) into a local configuration, `/var/lib/kubelet/instance-config.yaml`.
A kubelet configuration file is also written to `/etc/kubernetes/kubelet.conf`
with the baseline cluster-wide configuration for all kubelets in the cluster. This configuration file
points to the client certificates that allow the kubelet to communicate with the API server. This
addresses the need to
[propagate cluster-level configuration to each kubelet](#propagating-cluster-level-configuration-to-each-kubelet).
To address the second pattern of
[providing instance-specific configuration details](#providing-instance-specific-configuration-details),
kubeadm writes an environment file to `/var/lib/kubelet/kubeadm-flags.env`, which contains a list of
flags to pass to the kubelet when it starts. The flags are presented in the file like this:
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