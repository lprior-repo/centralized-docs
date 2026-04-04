---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#13-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 280
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