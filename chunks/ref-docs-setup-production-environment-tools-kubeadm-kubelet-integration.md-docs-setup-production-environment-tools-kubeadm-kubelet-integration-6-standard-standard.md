---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#6-standard
chunk_level: standard
chunk_type: prose
heading: Configure kubelets using kubeadm
token_count: 291
summary: ### Workflow when using `kubeadm join` When you run `kubeadm join`, kubeadm uses the Bootstrap Token credential to perform a TLS bootstrap, which fetches the credential needed to download the...
---

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