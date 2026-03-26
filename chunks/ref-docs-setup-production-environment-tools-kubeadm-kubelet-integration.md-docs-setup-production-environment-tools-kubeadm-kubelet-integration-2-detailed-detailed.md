---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 841
summary: ## Configure kubelets using kubeadm It is possible to configure the kubelet that kubeadm will start if a custom [`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/) API object...
---

## Configure kubelets using kubeadm
It is possible to configure the kubelet that kubeadm will start if a custom
[`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/)
API object is passed with a configuration file like so `kubeadm ... --config some-config-file.yaml`.
By calling `kubeadm config print init-defaults --component-configs KubeletConfiguration` you can
see all the default values for this structure.
It is also possible to apply instance-specific patches over the base `KubeletConfiguration`.
Have a look at [Customizing the kubelet](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/#customizing-the-kubelet)
for more details.
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
### Workflow when using `kubeadm join`
When you run `kubeadm join`, kubeadm uses the Bootstrap Token credential to perform
a TLS bootstrap, which fetches the credential needed to download the
`kubelet-config` ConfigMap and writes it to `/var/lib/kubelet/config.yaml`.
Additionally, the kubeadm tool detects the CRI socket on the node and writes its details
(including the socket path) into a local configuration, `/var/lib/kubelet/instance-config.yaml`.
The dynamic environment file is generated in exactly the same way as `kubeadm init`.
Next, `kubeadm` runs the following two commands to load the new configuration into the kubelet:
```
`systemctl daemon-reload &amp;&amp; systemctl restart kubelet
`
```
After the kubelet loads the new configuration, kubeadm writes the
`/etc/kubernetes/bootstrap-kubelet.conf` KubeConfig file, which contains a CA certificate and Bootstrap
Token. These are used by the kubelet to perform the TLS Bootstrap and obtain a unique
credential, which is stored in `/etc/kubernetes/kubelet.conf`.
When the `/etc/kubernetes/kubelet.conf` file is written, the kubelet has finished performing the TLS Bootstrap.
Kubeadm deletes the `/etc/kubernetes/bootstrap-kubelet.conf` file after completing the TLS Bootstrap.