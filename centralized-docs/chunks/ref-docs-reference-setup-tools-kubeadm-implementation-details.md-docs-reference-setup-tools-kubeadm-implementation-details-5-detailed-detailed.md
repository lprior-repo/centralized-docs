---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#5-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 668
summary: #### Caution: The generated configuration files include an embedded authentication key, and you should treat them as confidential. Also note that: 1. `ca.crt` certificate is embedded in all the...
---

#### Caution:
The generated configuration files include an embedded authentication key, and you should treat
them as confidential.
Also note that:
1. `ca.crt` certificate is embedded in all the kubeconfig files.
2. If a given kubeconfig file exists, and its content is evaluated as compliant with the above specs,
the existing file will be used and the generation phase for the given kubeconfig will be skipped
3. If kubeadm is running in [ExternalCA mode](/docs/reference/setup-tools/kubeadm/kubeadm-init/#external-ca-mode),
all the required kubeconfig must be provided by the user as well, because kubeadm cannot
generate any of them by itself
4. In case kubeadm is executed in the `--dry-run` mode, kubeconfig files are written in a temporary folder
5. Generation of kubeconfig files can be invoked individually with the
[`kubeadm init phase kubeconfig all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-kubeconfig) command### Generate static Pod manifests for control plane components
Kubeadm writes static Pod manifest files for control plane components to
`/etc/kubernetes/manifests`. The kubelet watches this directory for Pods to be created on startup.
Static Pod manifests share a set of common properties:
* All static Pods are deployed on `kube-system` namespace
* All static Pods get `tier:control-plane` and `component:{component-name}` labels
* All static Pods use the `system-node-critical` priority class
* `hostNetwork: true` is set on all static Pods to allow control plane startup before a network is
configured; as a consequence:
* The `address` that the controller-manager and the scheduler use to refer to the API server is `127.0.0.1`
* If the etcd server is set up locally, the `etcd-server` address will be set to `127.0.0.1:2379`
* Leader election is enabled for both the controller-manager and the scheduler
* Controller-manager and the scheduler will reference kubeconfig files with their respective, unique identities
* All static Pods get any extra flags or patches that you specify, as described in
[passing custom arguments to control plane components](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/)
* All static Pods get any extra Volumes specified by the user (Host path)
Please note that:
1. All images will be pulled from registry.k8s.io by default.
See [using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for customizing the image repository
2. In case kubeadm is executed in the `--dry-run` mode, static Pod files are written in a
temporary folder
3. Static Pod manifest generation for control plane components can be invoked individually with
the [`kubeadm init phase control-plane all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-control-plane) command#### API server
The static Pod manifest for the API server is affected by the following parameters provided by the users: